use crate::{ModelPricing, ProviderPricing, TokenPrices};

pub const GROQ: ProviderPricing = ProviderPricing {
	name: "groq",
	models: GROQ_MODELS,
};

const GROQ_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "gpt-oss-20b",
		prices: TokenPrices {
			input_normal: 0.075,
			input_cached: Some(0.037),
			input_cache_write: None,
			output_normal: 0.3,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "gpt-oss-120b",
		prices: TokenPrices {
			input_normal: 0.15,
			input_cached: Some(0.075),
			input_cache_write: None,
			output_normal: 0.6,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "meta-llama/llama-4-scout-17b-16e-instruct",
		prices: TokenPrices {
			input_normal: 0.11,
			input_cached: None,
			input_cache_write: None,
			output_normal: 0.34,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "qwen/qwen3-32b",
		prices: TokenPrices {
			input_normal: 0.29,
			input_cached: None,
			input_cache_write: None,
			output_normal: 0.59,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "llama-3.3-70b-versatile",
		prices: TokenPrices {
			input_normal: 0.59,
			input_cached: None,
			input_cache_write: None,
			output_normal: 0.79,
			output_reasoning: None,
		},
		tiers: &[],
	},
	ModelPricing {
		name: "llama-3.1-8b-instant",
		prices: TokenPrices {
			input_normal: 0.05,
			input_cached: None,
			input_cache_write: None,
			output_normal: 0.08,
			output_reasoning: None,
		},
		tiers: &[],
	},
];
