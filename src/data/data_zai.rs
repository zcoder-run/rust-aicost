use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const ZAI: ProviderPricing = ProviderPricing {
	name: "zai",
	models: ZAI_MODELS,
};

const ZAI_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "glm-5.3-flash",
		prices: TokenPrices {
			input_normal: 0.15,
			input_cached: Some(0.03),
			input_cache_write: None,
			output_normal: 0.5,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-5.3",
		prices: TokenPrices {
			input_normal: 1.4,
			input_cached: Some(0.26),
			input_cache_write: None,
			output_normal: 4.4,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-5.2",
		prices: TokenPrices {
			input_normal: 1.4,
			input_cached: Some(0.26),
			input_cache_write: None,
			output_normal: 4.4,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-5.1",
		prices: TokenPrices {
			input_normal: 1.4,
			input_cached: Some(0.26),
			input_cache_write: None,
			output_normal: 4.4,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-5",
		prices: TokenPrices {
			input_normal: 1.0,
			input_cached: Some(0.2),
			input_cache_write: None,
			output_normal: 3.2,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-5-turbo",
		prices: TokenPrices {
			input_normal: 1.2,
			input_cached: Some(0.24),
			input_cache_write: None,
			output_normal: 4.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.7",
		prices: TokenPrices {
			input_normal: 0.6,
			input_cached: Some(0.11),
			input_cache_write: None,
			output_normal: 2.2,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.7-flashx",
		prices: TokenPrices {
			input_normal: 0.07,
			input_cached: Some(0.01),
			input_cache_write: None,
			output_normal: 0.4,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.6",
		prices: TokenPrices {
			input_normal: 0.6,
			input_cached: Some(0.11),
			input_cache_write: None,
			output_normal: 2.2,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.5",
		prices: TokenPrices {
			input_normal: 0.6,
			input_cached: Some(0.11),
			input_cache_write: None,
			output_normal: 2.2,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.5-x",
		prices: TokenPrices {
			input_normal: 2.2,
			input_cached: Some(0.45),
			input_cache_write: None,
			output_normal: 8.9,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.5-air",
		prices: TokenPrices {
			input_normal: 0.2,
			input_cached: Some(0.03),
			input_cache_write: None,
			output_normal: 1.1,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.5-airx",
		prices: TokenPrices {
			input_normal: 1.1,
			input_cached: Some(0.22),
			input_cache_write: None,
			output_normal: 4.5,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4-32b-0414-128k",
		prices: TokenPrices {
			input_normal: 0.1,
			input_cached: None,
			input_cache_write: None,
			output_normal: 0.1,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.7-flash",
		prices: TokenPrices {
			input_normal: 0.0,
			input_cached: Some(0.0),
			input_cache_write: None,
			output_normal: 0.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "glm-4.5-flash",
		prices: TokenPrices {
			input_normal: 0.0,
			input_cached: Some(0.0),
			input_cache_write: None,
			output_normal: 0.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
