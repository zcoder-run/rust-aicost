use crate::data::PROVIDERS;
use crate::{AiCost, Error, ModelPricing, Result, TierInfo};
use genai::ModelIden;
use genai::chat::Usage;

/// Calculates the cost for a given provider type, model name, and usage.
pub fn compute(provider_type: &str, model_name: &str, usage: &Usage) -> Result<AiCost> {
	// Since not api from genai yet, extract the eventual namespace of the modelname
	let model_name = normalize_model_name(model_name);

	let provider_type = normalize_provider_type(provider_type);

	// Find the model within the provider (longest start_with)
	let model = resolve_model_entry(provider_type, model_name)
		.ok_or_else(|| Error::ModelNotFound(provider_type.to_string(), model_name.to_string()))?;

	Ok(compute_with_pricing(model, usage))
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

	resolve_model_entry(provider_type, model_name).copied()
}

// region:    --- Support

/// Computes the cost of `usage` with the given model pricing (tier resolved by prompt tokens).
fn compute_with_pricing(model: &ModelPricing, usage: &Usage) -> AiCost {
	// -- Resolve the prices for the prompt size tier
	let tier_prompt_tokens = usage.prompt_tokens.unwrap_or(0).max(0) as u32;
	let tier_index = model.tier_index_for(tier_prompt_tokens);
	let prices = tier_index.map(|i| &model.tiers[i].prices).unwrap_or(&model.prices);

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
	let price_prompt_normal = prices.input_normal;
	let price_prompt_cached = prices.input_cached.unwrap_or(price_prompt_normal);
	// Note: cache creation tokens fall back to the normal input price when no explicit input_cache_write price is set.
	// Without an explicit cache write price, cache creation tokens are folded back into the normal input tokens (no surcharge).
	let (prompt_tokens_normal, prompt_cache_creation_tokens, price_prompt_cache_creation) =
		match prices.input_cache_write {
			Some(price) => (prompt_tokens_normal, prompt_cache_creation_tokens, price),
			None => (
				prompt_tokens_normal + prompt_cache_creation_tokens,
				0.0,
				price_prompt_normal,
			),
		};

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
	let price_completion_normal = prices.output_normal;
	let price_completion_reasoning = prices.output_reasoning.unwrap_or(price_completion_normal);

	// -- Compute individual cost components (raw)
	let cost_prompt_normal = prompt_tokens_normal * price_prompt_normal / 1_000_000.0;
	let cost_prompt_cached = prompt_cached_tokens * price_prompt_cached / 1_000_000.0;
	let cost_prompt_cache_creation = prompt_cache_creation_tokens * price_prompt_cache_creation / 1_000_000.0;
	let cost_completion_normal = completion_tokens_normal * price_completion_normal / 1_000_000.0;
	let cost_completion_reasoning = completion_tokens_reasoning * price_completion_reasoning / 1_000_000.0;

	// -- Compute total components (rounded)
	let input_normal = (cost_prompt_normal * 1_000_000.0).round() / 1_000_000.0;
	let input_cache_read = (cost_prompt_cached * 1_000_000.0).round() / 1_000_000.0;
	let input_cache_write = (cost_prompt_cache_creation * 1_000_000.0).round() / 1_000_000.0;
	let input_total = ((input_normal + input_cache_read + input_cache_write) * 1_000_000.0).round() / 1_000_000.0;

	let output_normal = (cost_completion_normal * 1_000_000.0).round() / 1_000_000.0;
	let output_reasoning = (cost_completion_reasoning * 1_000_000.0).round() / 1_000_000.0;
	let output_total = ((output_normal + output_reasoning) * 1_000_000.0).round() / 1_000_000.0;

	let total = ((input_total + output_total) * 1_000_000.0).round() / 1_000_000.0;

	// -- Compute cache saving
	let input_cache_saving = if prompt_cached_tokens > 0.0 || prompt_cache_creation_tokens > 0.0 {
		let gross_saving = prompt_cached_tokens * (price_prompt_normal - price_prompt_cached) / 1_000_000.0;
		// Only the cache write premium over the normal input price reduces the saving.
		let cache_write_premium =
			prompt_cache_creation_tokens * (price_prompt_cache_creation - price_prompt_normal) / 1_000_000.0;
		let saving = gross_saving - cache_write_premium;
		(saving * 1_000_000.0).round() / 1_000_000.0
	} else {
		0.0
	};

	AiCost {
		total,

		// -- input
		input_total,
		input_normal,
		input_cache_read,
		input_cache_write,
		input_cache_saving,

		// -- output
		output_total,
		output_normal,
		output_reasoning,

		// -- tier
		tier: tier_index.map(|i| TierInfo { tier_index: i }),
	}
}

fn normalize_model_name(model_name: &str) -> &str {
	match model_name.split_once("::") {
		Some((_, after)) => after,
		None => model_name,
	}
}

fn normalize_provider_type(provider_type: &str) -> &str {
	if provider_type == "openai_resp" {
		"openai"
	} else if provider_type == "gemini_ix" {
		"gemini"
	} else {
		provider_type
	}
}

fn resolve_model_entry(provider_type: &str, model_name: &str) -> Option<&'static ModelPricing> {
	if is_bedrock(provider_type) {
		find_bedrock_model_entry(model_name)
	} else {
		find_model_entry(provider_type, model_name)
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

fn is_bedrock(provider_type: &str) -> bool {
	matches!(provider_type, "bedrock_api" | "bedrock_sigv4")
}

fn find_bedrock_model_entry(model_name: &str) -> Option<&'static ModelPricing> {
	let (provider, model_name) = bedrock_pricing_target(model_name)?;
	find_model_entry(provider, model_name)
}

fn bedrock_pricing_target(model_name: &str) -> Option<(&str, &str)> {
	let (prefix, suffix) = model_name.split_once('.')?;
	let (provider, model_name) = match prefix {
		"openai" | "anthropic" => (prefix, suffix),
		"" => return None,
		_ => suffix.split_once('.')?,
	};

	match provider {
		"openai" | "anthropic" if !model_name.is_empty() => Some((provider, model_name)),
		_ => None,
	}
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{PricingTier, TokenPrices};
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
		let price = ai_cost.total;
		// gpt-4o pricing: input_normal: 2.5, output_normal: 10.0
		// Expected: (1000 * 2.5 / 1_000_000) + (500 * 10.0 / 1_000_000) = 0.0025 + 0.005 = 0.0075
		let expected = 0.0075;
		assert!((price - expected).abs() < f64::EPSILON);
		assert!(ai_cost.input_cache_write == 0.0);
		assert!(ai_cost.input_cache_saving == 0.0);
		assert_eq!(ai_cost.tier, None);

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
		let price = ai_cost.total;
		// gpt-4o-mini pricing: input_normal: 0.15, input_cached: 0.075, output_normal: 0.6
		let cached = fx_cached_tokens as f64 * 0.075 / 1_000_000.0;
		let prompt = fx_prompt_normal_tokens as f64 * 0.150 / 1_000_000.0;
		let completion = fx_completion_tokens as f64 * 0.6 / 1_000_000.0;
		let expected = cached + prompt + completion;
		let expected = (expected * 1_000_000.0).round() / 1_000_000.0;
		assert!((price - expected).abs() < f64::EPSILON);
		assert!(ai_cost.input_cache_saving > 0.0);
		assert!(ai_cost.input_cache_write == 0.0);

		Ok(())
	}

	#[test]
	fn test_pricing_core_cost_with_cache_creation() -> TestResult {
		let usage = Usage {
			prompt_tokens: Some(600),
			prompt_tokens_details: Some(PromptTokensDetails {
				cached_tokens: Some(400),
				cache_creation_tokens: Some(200),
				..Default::default()
			}),
			..Default::default()
		};

		let ai_cost = compute("openai", "gpt-4o-mini", &usage)?;

		let gross_saving = 400.0 * (0.15 - 0.075) / 1_000_000.0;
		let expected = (gross_saving * 1_000_000.0_f64).round() / 1_000_000.0;
		assert!((ai_cost.input_cache_saving - expected).abs() < f64::EPSILON);
		// No explicit cache write price, so cache creation tokens are billed as normal input (no surcharge).
		assert!(ai_cost.input_cache_write == 0.0);
		assert!((ai_cost.input_normal - 200.0 * 0.15 / 1_000_000.0).abs() < 1e-9);

		Ok(())
	}

	#[test]
	fn test_cost_compute_zai_glm_5_3_flash() -> TestResult {
		// -- Setup & Fixtures
		let usage = Usage {
			prompt_tokens: Some(1000),
			completion_tokens: Some(500),
			prompt_tokens_details: None,
			..Default::default()
		};
		let fx_expected_input = 1000. * 0.15 / 1_000_000.;
		let fx_expected_output = 500. * 0.50 / 1_000_000.;
		let fx_total = fx_expected_input + fx_expected_output;

		// -- Exec
		let ai_cost = compute("zai", "glm-5.3-flash", &usage)?;

		// -- Check
		let price = ai_cost.total;
		assert!((price - fx_total).abs() < f64::EPSILON);
		assert_eq!(ai_cost.input_normal, fx_expected_input);
		assert_eq!(ai_cost.output_normal, fx_expected_output);

		Ok(())
	}

	#[test]
	fn test_cost_compute_with_pricing_cache_write_explicit() -> TestResult {
		// -- Setup & Fixtures
		let usage = Usage {
			prompt_tokens: Some(600),
			prompt_tokens_details: Some(PromptTokensDetails {
				cache_creation_tokens: Some(200),
				..Default::default()
			}),
			..Default::default()
		};

		// -- Exec
		let ai_cost = compute_with_pricing(&FX_MODEL, &usage);

		// -- Check
		assert!((ai_cost.input_cache_write - 0.0004).abs() < 1e-9);
		assert!((ai_cost.input_normal - 0.0004).abs() < 1e-9);
		assert!((ai_cost.input_cache_saving + 0.0002).abs() < 1e-9);

		Ok(())
	}

	#[test]
	fn test_cost_compute_with_pricing_cache_write_fallback() -> TestResult {
		// -- Setup & Fixtures
		let usage = Usage {
			prompt_tokens: Some(600),
			prompt_tokens_details: Some(PromptTokensDetails {
				cache_creation_tokens: Some(200),
				..Default::default()
			}),
			..Default::default()
		};

		// -- Exec
		let ai_cost = compute_with_pricing(&FX_MODEL_NO_CACHE_WRITE, &usage);

		// -- Check
		assert!(ai_cost.input_cache_write == 0.0);
		assert!((ai_cost.input_normal - 0.0006).abs() < 1e-9);
		assert!(ai_cost.input_cache_saving == 0.0);

		Ok(())
	}

	#[test]
	fn test_cost_compute_with_pricing_tier_at_threshold() -> TestResult {
		// -- Setup & Fixtures
		let usage = Usage {
			prompt_tokens: Some(1000),
			completion_tokens: Some(100),
			..Default::default()
		};

		// -- Exec
		let ai_cost = compute_with_pricing(&FX_MODEL, &usage);

		// -- Check
		assert!((ai_cost.input_normal - 0.001).abs() < 1e-9);
		assert!((ai_cost.output_normal - 0.0005).abs() < 1e-9);
		assert!((ai_cost.total - 0.0015).abs() < 1e-9);
		assert_eq!(ai_cost.tier, None);

		Ok(())
	}

	#[test]
	fn test_cost_compute_with_pricing_tier_above_threshold() -> TestResult {
		// -- Setup & Fixtures
		let usage = Usage {
			prompt_tokens: Some(1001),
			completion_tokens: Some(100),
			..Default::default()
		};

		// -- Exec
		let ai_cost = compute_with_pricing(&FX_MODEL, &usage);

		// -- Check
		assert!((ai_cost.input_normal - 0.002002).abs() < 1e-9);
		assert!((ai_cost.output_normal - 0.001).abs() < 1e-9);
		assert!((ai_cost.total - 0.003002).abs() < 1e-9);
		assert_eq!(ai_cost.tier, Some(TierInfo { tier_index: 0 }));

		Ok(())
	}

	#[test]
	fn test_cost_bedrock_pricing_target_model_ids() -> TestResult {
		// -- Setup & Fixtures
		let cases = [
			("global.openai.gpt-6-luna", Some(("openai", "gpt-6-luna"))),
			("us.anthropic.claude-haiku-5-5", Some(("anthropic", "claude-haiku-5-5"))),
			("openai.gpt-6-luna.v1", Some(("openai", "gpt-6-luna.v1"))),
			("anthropic.claude-haiku-5-5", Some(("anthropic", "claude-haiku-5-5"))),
			("us.meta.llama", None),
			("us.global.openai.gpt-6-luna", None),
			("openai.", None),
		];

		// -- Exec & Check
		for (model_name, expected) in cases {
			assert_eq!(bedrock_pricing_target(model_name), expected, "{model_name}");
		}

		Ok(())
	}

	#[test]
	fn test_cost_compute_bedrock_upstream_pricing() -> TestResult {
		// -- Setup & Fixtures
		let anthropic = PROVIDERS
			.iter()
			.find(|provider| provider.name == "anthropic")
			.ok_or("Should have Anthropic pricing")?;
		let anthropic_model = anthropic
			.models
			.iter()
			.find(|model| !model.tiers.is_empty())
			.or_else(|| anthropic.models.first())
			.ok_or("Should have an Anthropic model")?;
		let cases = [
			("global", "openai", "gpt-4o-mini", 1_000, None),
			(
				"us",
				"anthropic",
				anthropic_model.name,
				anthropic_model.tiers.first().map_or(1_000, |tier| tier.above_prompt_tokens + 1),
				anthropic_model.tiers.first().map(|_| TierInfo { tier_index: 0 }),
			),
		];

		// -- Exec & Check
		for (profile, provider, model, prompt_tokens, expected_tier) in cases {
			let usage = Usage {
				prompt_tokens: Some(prompt_tokens.try_into()?),
				completion_tokens: Some(500),
				prompt_tokens_details: Some(PromptTokensDetails {
					cached_tokens: Some(200),
					cache_creation_tokens: Some(200),
					..Default::default()
				}),
				..Default::default()
			};
			let direct_cost = compute(provider, model, &usage)?;
			assert_eq!(direct_cost.tier, expected_tier);

			for adapter in ["bedrock_api", "bedrock_sigv4"] {
				let model_id = format!("{adapter}::{profile}.{provider}.{model}");
				let bedrock_cost = compute(adapter, &model_id, &usage)?;

				assert_eq!(bedrock_cost.total, direct_cost.total, "{model_id}");
				assert_eq!(
					bedrock_cost.input_cache_read, direct_cost.input_cache_read,
					"{model_id}"
				);
				assert_eq!(
					bedrock_cost.input_cache_write, direct_cost.input_cache_write,
					"{model_id}"
				);
				assert_eq!(bedrock_cost.tier, direct_cost.tier, "{model_id}");
			}
		}

		Ok(())
	}

	#[test]
	fn test_cost_compute_bedrock_model_not_found() -> TestResult {
		// -- Setup & Fixtures
		let usage = Usage::default();
		let cases = ["us.meta.llama", "global.openai.unknown-model", "gpt-4o-mini"];

		// -- Exec & Check
		for adapter in ["bedrock_api", "bedrock_sigv4"] {
			for model_name in cases {
				let model_id = format!("{adapter}::{model_name}");
				let result = compute(adapter, &model_id, &usage);

				assert!(matches!(
					result,
					Err(Error::ModelNotFound(provider, model)) if provider == adapter && model == model_name
				));
			}
		}

		Ok(())
	}

	// -- Test Support
	const FX_TIERS: &[PricingTier] = &[PricingTier {
		above_prompt_tokens: 1_000,
		prices: TokenPrices {
			input_normal: 2.0,
			input_cached: Some(0.2),
			input_cache_write: Some(4.0),
			output_normal: 10.0,
			output_reasoning: None,
		},
	}];

	const FX_MODEL: ModelPricing = ModelPricing {
		name: "fx-model",
		prices: TokenPrices {
			input_normal: 1.0,
			input_cached: Some(0.1),
			input_cache_write: Some(2.0),
			output_normal: 5.0,
			output_reasoning: None,
		},
		tiers: FX_TIERS,
	};

	const FX_MODEL_NO_CACHE_WRITE: ModelPricing = ModelPricing {
		name: "fx-model-no-cache-write",
		prices: TokenPrices {
			input_normal: 1.0,
			input_cached: Some(0.1),
			input_cache_write: None,
			output_normal: 5.0,
			output_reasoning: None,
		},
		tiers: &[],
	};
}

// endregion: --- Tests
