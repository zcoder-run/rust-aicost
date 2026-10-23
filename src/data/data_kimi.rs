use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const KIMI: ProviderPricing = ProviderPricing {
	name: "kimi",
	models: KIMI_MODELS,
};

const KIMI_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "kimi-k3.1-pro",
		prices: TokenPrices {
			input_normal: 2.0,
			input_cached: Some(0.2),
			input_cache_write: None,
			output_normal: 10.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "kimi-k3.1-pro-highspeed",
		prices: TokenPrices {
			input_normal: 4.0,
			input_cached: Some(0.2),
			input_cache_write: None,
			output_normal: 20.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "kimi-k3",
		prices: TokenPrices {
			input_normal: 3.0,
			input_cached: Some(0.3),
			input_cache_write: None,
			output_normal: 15.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "kimi-k2.7-code",
		prices: TokenPrices {
			input_normal: 0.95,
			input_cached: Some(0.19),
			input_cache_write: None,
			output_normal: 4.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "kimi-k2.7-code-highspeed",
		prices: TokenPrices {
			input_normal: 1.9,
			input_cached: Some(0.38),
			input_cache_write: None,
			output_normal: 8.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "kimi-k2.6",
		prices: TokenPrices {
			input_normal: 0.95,
			input_cached: Some(0.16),
			input_cache_write: None,
			output_normal: 4.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "kimi-k2.5",
		prices: TokenPrices {
			input_normal: 0.6,
			input_cached: Some(0.1),
			input_cache_write: None,
			output_normal: 3.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
