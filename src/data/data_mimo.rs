use crate::{ModelPricing, ProviderPricing};

pub const MIMO: ProviderPricing = ProviderPricing {
	name: "mimo",
	models: MIMO_MODELS,
};

const MIMO_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "mimo-v2.5-pro",
		input_normal: 0.435,
		input_cached: Some(0.0036),
		output_normal: 0.87,
		output_reasoning: None,
	},
	ModelPricing {
		name: "mimo-v2.5",
		input_normal: 0.14,
		input_cached: Some(0.0028),
		output_normal: 0.28,
		output_reasoning: None,
	},
];
