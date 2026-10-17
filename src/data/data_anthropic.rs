use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const ANTHROPIC: ProviderPricing = ProviderPricing {
	name: "anthropic",
	models: ANTHROPIC_MODELS,
};

pub const ANTHROPIC_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "claude-fable-5-1",
		prices: TokenPrices {
			input_normal: 10.0,
			input_cached: Some(0.25),
			input_cache_write: None,
			output_normal: 50.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-mythos-5-1",
		prices: TokenPrices {
			input_normal: 10.0,
			input_cached: Some(0.25),
			input_cache_write: None,
			output_normal: 50.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-fable-5",
		prices: TokenPrices {
			input_normal: 10.0,
			input_cached: Some(1.0),
			input_cache_write: None,
			output_normal: 50.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-mythos-5",
		prices: TokenPrices {
			input_normal: 10.0,
			input_cached: Some(1.0),
			input_cache_write: None,
			output_normal: 50.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-opus-5-5",
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
		name: "claude-opus-5",
		prices: TokenPrices {
			input_normal: 5.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 25.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-opus-4-8",
		prices: TokenPrices {
			input_normal: 5.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 25.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-opus-4-7",
		prices: TokenPrices {
			input_normal: 5.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 25.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-opus-4-6",
		prices: TokenPrices {
			input_normal: 5.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 25.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-opus-4-5",
		prices: TokenPrices {
			input_normal: 5.0,
			input_cached: Some(0.5),
			input_cache_write: None,
			output_normal: 25.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-sonnet-5-5",
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
		name: "claude-sonnet-5",
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
		name: "claude-sonnet-4-6",
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
		name: "claude-sonnet-4-5",
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
		name: "claude-haiku-5-5",
		prices: TokenPrices {
			input_normal: 0.1,
			input_cached: Some(0.01),
			input_cache_write: None,
			output_normal: 0.5,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "claude-haiku-4-5",
		prices: TokenPrices {
			input_normal: 1.0,
			input_cached: Some(0.1),
			input_cache_write: None,
			output_normal: 5.0,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
