// region:    --- Types

#[derive(Debug)]
pub struct ProviderPricing {
	pub name: &'static str,
	pub models: &'static [ModelPricing],
}

#[derive(Debug, Clone, Copy)]
pub struct ModelPricing {
	pub name: &'static str,
	pub prices: TokenPrices,

	/// Higher price tiers, sorted by ascending `above_prompt_tokens`. Empty for most models.
	/// The whole request is billed at the selected tier (not a marginal rate).
	pub tiers: &'static [PricingTier],
}

#[derive(Debug, Clone, Copy)]
pub struct PricingTier {
	/// Tier applies when the total prompt tokens are strictly greater than this value.
	pub above_prompt_tokens: u32,
	pub prices: TokenPrices,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenPrices {
	pub input_normal: f64,
	pub input_cached: Option<f64>,
	pub input_cache_write: Option<f64>,
	pub output_normal: f64,
	pub output_reasoning: Option<f64>,
}

// endregion: --- Types

impl ModelPricing {
	/// Returns the highest tier whose threshold is strictly exceeded, or `None` for the base prices.
	pub fn tier_for(&self, prompt_tokens: u32) -> Option<&PricingTier> {
		self.tiers.iter().rev().find(|t| prompt_tokens > t.above_prompt_tokens)
	}

	/// Returns the prices of the highest tier whose threshold is strictly exceeded, or the base prices.
	pub fn prices_for(&self, prompt_tokens: u32) -> &TokenPrices {
		self.tier_for(prompt_tokens).map(|t| &t.prices).unwrap_or(&self.prices)
	}
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;

	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	const FX_BASE: TokenPrices = TokenPrices {
		input_normal: 1.0,
		input_cached: Some(0.1),
		input_cache_write: Some(1.25),
		output_normal: 5.0,
		output_reasoning: None,
	};

	const FX_TIERS: &[PricingTier] = &[
		PricingTier {
			above_prompt_tokens: 1_000,
			prices: TokenPrices {
				input_normal: 2.0,
				..FX_BASE
			},
		},
		PricingTier {
			above_prompt_tokens: 2_000,
			prices: TokenPrices {
				input_normal: 3.0,
				..FX_BASE
			},
		},
	];

	const FX_MODEL: ModelPricing = ModelPricing {
		name: "fx-model",
		prices: FX_BASE,
		tiers: FX_TIERS,
	};

	const FX_MODEL_NO_TIERS: ModelPricing = ModelPricing {
		name: "fx-model",
		prices: FX_BASE,
		tiers: &[],
	};

	#[test]
	fn test_model_model_pricing_prices_for_base_at_threshold() -> Result<()> {
		// -- Setup & Fixtures
		let model = FX_MODEL;

		// -- Exec
		let prices = model.prices_for(1_000);

		// -- Check
		assert_eq!(prices.input_normal, 1.0);

		Ok(())
	}

	#[test]
	fn test_model_model_pricing_prices_for_tier_above_threshold() -> Result<()> {
		// -- Setup & Fixtures
		let model = FX_MODEL;

		// -- Exec
		let prices = model.prices_for(1_001);

		// -- Check
		assert_eq!(prices.input_normal, 2.0);

		Ok(())
	}

	#[test]
	fn test_model_model_pricing_prices_for_highest_exceeded_tier() -> Result<()> {
		// -- Setup & Fixtures
		let model = FX_MODEL;

		// -- Exec
		let at_second_threshold = model.prices_for(2_000);
		let above_second_threshold = model.prices_for(2_001);

		// -- Check
		assert_eq!(at_second_threshold.input_normal, 2.0);
		assert_eq!(above_second_threshold.input_normal, 3.0);

		Ok(())
	}

	#[test]
	fn test_model_model_pricing_prices_for_no_tiers() -> Result<()> {
		// -- Setup & Fixtures
		let model = FX_MODEL_NO_TIERS;

		// -- Exec
		let prices = model.prices_for(u32::MAX);

		// -- Check
		assert_eq!(prices.input_normal, 1.0);

		Ok(())
	}

	#[test]
	fn test_model_model_pricing_tier_for_none_at_threshold() -> Result<()> {
		// -- Setup & Fixtures
		let model = FX_MODEL;

		// -- Exec
		let tier = model.tier_for(1_000);

		// -- Check
		assert!(tier.is_none());

		Ok(())
	}

	#[test]
	fn test_model_model_pricing_tier_for_above_threshold() -> Result<()> {
		// -- Setup & Fixtures
		let model = FX_MODEL;

		// -- Exec
		let tier = model.tier_for(1_001).ok_or("Should have a tier above 1_000")?;

		// -- Check
		assert_eq!(tier.above_prompt_tokens, 1_000);

		Ok(())
	}

	#[test]
	fn test_model_model_pricing_tier_for_no_tiers() -> Result<()> {
		// -- Setup & Fixtures
		let model = FX_MODEL_NO_TIERS;

		// -- Exec
		let tier = model.tier_for(u32::MAX);

		// -- Check
		assert!(tier.is_none());

		Ok(())
	}
}

// endregion: --- Tests
