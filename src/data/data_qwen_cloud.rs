use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const QWEN_CLOUD: ProviderPricing = ProviderPricing {
	name: "qwen_cloud",
	models: QWEN_CLOUD_MODELS,
};

const QWEN_CLOUD_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "qwen3.8-max",
		prices: TokenPrices {
			input_normal: 2.0,
			input_cached: Some(0.25),
			input_cache_write: None,
			output_normal: 6.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "qwen3.7-plus",
		prices: TokenPrices {
			input_normal: 0.32,
			input_cached: Some(0.064),
			input_cache_write: None,
			output_normal: 1.28,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
