use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const MIMO: ProviderPricing = ProviderPricing {
	name: "mimo",
	models: MIMO_MODELS,
};

const MIMO_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "mimo-v2.5-pro",
		prices: TokenPrices {
			input_normal: 0.435,
			input_cached: Some(0.0036),
			input_cache_write: None,
			output_normal: 0.87,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "mimo-v2.5",
		prices: TokenPrices {
			input_normal: 0.14,
			input_cached: Some(0.0028),
			input_cache_write: None,
			output_normal: 0.28,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
