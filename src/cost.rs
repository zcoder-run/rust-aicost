use crate::data::PROVIDERS;
use crate::{AiCost, Error, ModelPricing, Result};
use genai::ModelIden;
use genai::chat::Usage;

/// Calculates the cost for a given provider type, model name, and usage.
pub fn compute(provider_type: &str, model_name: &str, usage: &Usage) -> Result<AiCost> {
	// Since not api from genai yet, extract the eventual namespace of the modelname
	let model_name = normalize_model_name(model_name);

	let provider_type = normalize_provider_type(provider_type);

	// Find the model within the provider (longest start_with)
	let model = find_model_entry(provider_type, model_name)
		.ok_or_else(|| Error::ModelNotFound(provider_type.to_string(), model_name.to_string()))?;

	// -- Extract prompt related tokens and pricing
	let prompt_tokens = usage.prompt_tokens.unwrap_or(0) as f64;
	// Extract cached vs normal prompt tokens
	let (prompt_tokens_normal, prompt_cached_tokens, prompt_cache_creation_tokens) = match &usage.prompt_tokens_details
	{
		Some(details) => {
			let cached = details.cached_tokens.unwrap_or(0) as f64;
			let cache_creation_tokens = details.cache_creation_tokens.unwrap_or(0) as f64;
			let normal = prompt_tokens - cached - cache_creation_tokens;
			(normal, cached, cache_creation_tokens)
		}
		None => (prompt_tokens, 0.0, 0.0),
	};
	// Extract prompt prices
	let price_prompt_normal = model.input_normal;
	let price_prompt_cached = model.input_cached.unwrap_or(price_prompt_normal);
	// Note:  For now, for cache_creation_tokens assume * 1.25 for cache_creation_tokens (which is Anthropic rules, and this is only anthropic data)
	let price_prompt_cache_creation = 1.25 * price_prompt_normal;

	// -- Extract completion related tokens and pricing
	let completion_tokens = usage.completion_tokens.unwrap_or(0) as f64;
	let (completion_tokens_normal, completion_tokens_reasoning) = if let Some(reasoning_tokens) = usage
		.completion_tokens_details
		.as_ref()
		.and_then(|v| v.reasoning_tokens.map(|v| v as f64))
	{
		(completion_tokens - reasoning_tokens, reasoning_tokens)
	} else {
		(completion_tokens, 0.)
	};
	let price_completion_normal = model.output_normal;
	let price_completion_reasoning = model.output_reasoning.unwrap_or(price_completion_normal);

	// -- Compute individual cost components
	let cost_prompt_normal = prompt_tokens_normal * price_prompt_normal / 1_000_000.0;
	let cost_prompt_cached = prompt_cached_tokens * price_prompt_cached / 1_000_000.0;
	let cost_prompt_cache_creation = prompt_cache_creation_tokens * price_prompt_cache_creation / 1_000_000.0;
	let cost_completion_normal = completion_tokens_normal * price_completion_normal / 1_000_000.0;
	let cost_completion_reasoning = completion_tokens_reasoning * price_completion_reasoning / 1_000_000.0;

	// -- Compute total cost
	let cost = cost_prompt_normal
		+ cost_prompt_cached
		+ cost_prompt_cache_creation
		+ cost_completion_normal
		+ cost_completion_reasoning;
	let cost = (cost * 1_000_000.0).round() / 1_000_000.0;

	// -- Compute cache write cost (only if there was cache creation)
	let cost_cache_write = if prompt_cache_creation_tokens > 0.0 {
		Some((cost_prompt_cache_creation * 1_000_000.0).round() / 1_000_000.0)
	} else {
		None
	};

	// -- Compute cache saving (difference between what cached tokens would have cost at normal price vs cached price)
	let cost_cache_saving = if prompt_cached_tokens > 0.0 {
		let would_have_cost = prompt_cached_tokens * price_prompt_normal / 1_000_000.0;
		let saving = would_have_cost - cost_prompt_cached;
		if saving > 0.0 {
			Some((saving * 1_000_000.0).round() / 1_000_000.0)
		} else {
			None
		}
	} else {
		None
	};

	Ok(AiCost {
		cost,
		cost_cache_write,
		cost_cache_saving,
	})
}

/// Calculates the cost for a given `ModelIden` and usage.
pub fn compute_iden(model_iden: &ModelIden, usage: &Usage) -> Result<AiCost> {
	let provider_type = model_iden.adapter_kind.as_lower_str();
	let model_name = &*model_iden.model_name;

	compute(provider_type, model_name, usage)
}

pub fn model_pricing(model_iden: &ModelIden) -> Option<ModelPricing> {
	let provider_type = model_iden.adapter_kind.as_lower_str();
	let model_name = &*model_iden.model_name;

	let model_name = normalize_model_name(model_name);

	let provider_type = normalize_provider_type(provider_type);

	find_model_entry(provider_type, model_name).copied()
}

// region:    --- Support

fn normalize_model_name(model_name: &str) -> &str {
	match model_name.split_once("::") {
		Some((_, after)) => after,
		None => model_name,
	}
}

fn normalize_provider_type(provider_type: &str) -> &str {
	if provider_type == "openai_resp" {
		"openai"
	} else {
		provider_type
	}
}

fn find_model_entry(provider_type: &str, model_name: &str) -> Option<&'static ModelPricing> {
	let provider = PROVIDERS.iter().find(|p| p.name == provider_type)?;

	let mut model: Option<&ModelPricing> = None;
	for m in provider.models.iter() {
		if model_name.starts_with(m.name) {
			let current_len = model.map(|m| m.name.len()).unwrap_or(0);
			if current_len < m.name.len() {
				model = Some(m)
			}
		}
	}

	model
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use genai::chat::{PromptTokensDetails, Usage};

	type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

	#[test]
	fn test_pricing_core_cost_simple() -> TestResult {
		// -- Setup & Fixtures
		let usage = Usage {
			prompt_tokens: Some(1000),
			completion_tokens: Some(500),
			prompt_tokens_details: None,
			..Default::default()
		};

		// -- Exec
		let ai_cost = compute("openai", "gpt-4o", &usage)?;

		// -- Check
		let price = ai_cost.cost;
		// gpt-4o pricing: input_normal: 2.5, output_normal: 10.0
		// Expected: (1000 * 2.5 / 1_000_000) + (500 * 10.0 / 1_000_000) = 0.0025 + 0.005 = 0.0075
		let expected = 0.0075;
		assert!((price - expected).abs() < f64::EPSILON);
		assert!(ai_cost.cost_cache_write.is_none());
		assert!(ai_cost.cost_cache_saving.is_none());

		Ok(())
	}

	#[test]
	fn test_pricing_core_cost_with_cached() -> TestResult {
		// -- Setup & Fixtures
		let fx_prompt_normal_tokens = 1000;
		let fx_completion_tokens = 500;
		let fx_cached_tokens = 400;
		let usage = Usage {
			prompt_tokens: Some(fx_prompt_normal_tokens + fx_cached_tokens),
			completion_tokens: Some(fx_completion_tokens),
			prompt_tokens_details: Some(PromptTokensDetails {
				cached_tokens: Some(fx_cached_tokens),
				..Default::default()
			}),
			..Default::default()
		};

		// -- Exec
		let ai_cost = compute("openai", "gpt-4o-mini", &usage)?;

		// -- Check
		let price = ai_cost.cost;
		// gpt-4o-mini pricing: input_normal: 0.15, input_cached: 0.075, output_normal: 0.6
		let cached = fx_cached_tokens as f64 * 0.075 / 1_000_000.0;
		let prompt = fx_prompt_normal_tokens as f64 * 0.150 / 1_000_000.0;
		let completion = fx_completion_tokens as f64 * 0.6 / 1_000_000.0;
		let expected = cached + prompt + completion;
		let expected = (expected * 1_000_000.0).round() / 1_000_000.0;
		assert!((price - expected).abs() < f64::EPSILON);
		assert!(ai_cost.cost_cache_saving.is_some());
		assert!(ai_cost.cost_cache_write.is_none());

		Ok(())
	}
}

// endregion: --- Tests
