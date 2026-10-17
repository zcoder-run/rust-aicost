use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const DEEPSEEK: ProviderPricing = ProviderPricing {
	name: "deepseek",
	models: DEEPSEEK_MODELS,
};

const DEEPSEEK_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "deepseek-flash",
		prices: TokenPrices {
			input_normal: 0.15,
			input_cached: Some(0.003),
			input_cache_write: None,
			output_normal: 0.6,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "deepseek-v4-flash",
		prices: TokenPrices {
			input_normal: 0.22,
			input_cached: Some(0.007),
			input_cache_write: None,
			output_normal: 0.66,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "deepseek-v4-pro",
		prices: TokenPrices {
			input_normal: 0.66,
			input_cached: Some(0.022),
			input_cache_write: None,
			output_normal: 1.98,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "deepseek-chat",
		prices: TokenPrices {
			input_normal: 0.27,
			input_cached: Some(0.07),
			input_cache_write: None,
			output_normal: 1.1,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "deepseek-reasoner",
		prices: TokenPrices {
			input_normal: 0.55,
			input_cached: Some(0.14),
			input_cache_write: None,
			output_normal: 2.19,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
