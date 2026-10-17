use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const MINIMAX: ProviderPricing = ProviderPricing {
	name: "minimax",
	models: MINIMAX_MODELS,
};

const MINIMAX_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "MiniMax-M3",
		prices: TokenPrices {
			input_normal: 0.3,
			input_cached: Some(0.06),
			input_cache_write: None,
			output_normal: 1.2,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "MiniMax-M2.7",
		prices: TokenPrices {
			input_normal: 0.3,
			input_cached: Some(0.06),
			input_cache_write: None,
			output_normal: 1.2,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "MiniMax-M2.7-highspeed",
		prices: TokenPrices {
			input_normal: 0.6,
			input_cached: Some(0.06),
			input_cache_write: None,
			output_normal: 2.4,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
