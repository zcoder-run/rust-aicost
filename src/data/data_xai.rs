use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const XAI: ProviderPricing = ProviderPricing {
	name: "xai",
	models: XAI_MODELS,
};

const XAI_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "grok-4.7",
		prices: TokenPrices {
			input_normal: 2.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 6.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "grok-4.6",
		prices: TokenPrices {
			input_normal: 2.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 6.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "grok-4.5",
		prices: TokenPrices {
			input_normal: 2.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 6.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "grok-4.3",
		prices: TokenPrices {
			input_normal: 1.25,
			input_cached: Some(0.2),
			input_cache_write: None,
			output_normal: 2.5,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "grok-build-0.1",
		prices: TokenPrices {
			input_normal: 1.0,
			input_cached: Some(0.2),
			input_cache_write: None,
			output_normal: 2.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
