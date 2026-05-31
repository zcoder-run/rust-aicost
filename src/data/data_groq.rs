use crate::{ModelPricing, ProviderPricing};

pub const GROQ: ProviderPricing = ProviderPricing {
	name: "groq",
	models: GROQ_MODELS,
};

const GROQ_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "gpt-oss-20b",
		input_cached: Some(0.037),
		input_normal: 0.075,
		output_normal: 0.3,
		output_reasoning: None,
	},
	ModelPricing {
		name: "gpt-oss-120b",
		input_cached: Some(0.075),
		input_normal: 0.15,
		output_normal: 0.6,
		output_reasoning: None,
	},
	ModelPricing {
		name: "meta-llama/llama-4-scout-17b-16e-instruct",
		input_cached: None,
		input_normal: 0.11,
		output_normal: 0.34,
		output_reasoning: None,
	},
	ModelPricing {
		name: "qwen/qwen3-32b",
		input_cached: None,
		input_normal: 0.29,
		output_normal: 0.59,
		output_reasoning: None,
	},
	ModelPricing {
		name: "llama-3.3-70b-versatile",
		input_cached: None,
		input_normal: 0.59,
		output_normal: 0.79,
		output_reasoning: None,
	},
	ModelPricing {
		name: "llama-3.1-8b-instant",
		input_cached: None,
		input_normal: 0.05,
		output_normal: 0.08,
		output_reasoning: None,
	},
];
