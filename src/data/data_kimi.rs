use crate::{ModelPricing, ProviderPricing};

pub const KIMI: ProviderPricing = ProviderPricing {
	name: "kimi",
	models: KIMI_MODELS,
};

const KIMI_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "kimi-k2.7-code",
		input_cached: Some(0.19),
		input_normal: 0.95,
		output_normal: 4.0,
		output_reasoning: None,
	},
	ModelPricing {
		name: "kimi-k2.7-code-highspeed",
		input_cached: Some(0.38),
		input_normal: 1.9,
		output_normal: 8.0,
		output_reasoning: None,
	},
];
