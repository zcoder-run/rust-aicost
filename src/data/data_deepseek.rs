use crate::{ModelPricing, ProviderPricing};

pub const DEEPSEEK: ProviderPricing = ProviderPricing {
	name: "deepseek",
	models: DEEPSEEK_MODELS,
};

const DEEPSEEK_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "deepseek-v4.1-flash",
		input_cached: Some(0.003),
		input_normal: 0.15,
		output_normal: 0.6,
		output_reasoning: None,
	},
	ModelPricing {
		name: "deepseek-v4-flash",
		input_cached: Some(0.007),
		input_normal: 0.22,
		output_normal: 0.66,
		output_reasoning: None,
	},
	ModelPricing {
		name: "deepseek-v4-pro",
		input_cached: Some(0.022),
		input_normal: 0.66,
		output_normal: 1.98,
		output_reasoning: None,
	},
	ModelPricing {
		name: "deepseek-chat",
		input_cached: Some(0.07),
		input_normal: 0.27,
		output_normal: 1.1,
		output_reasoning: None,
	},
	ModelPricing {
		name: "deepseek-reasoner",
		input_cached: Some(0.14),
		input_normal: 0.55,
		output_normal: 2.19,
		output_reasoning: None,
	},
];
