use crate::{ModelPricing, ProviderPricing};

pub const XAI: ProviderPricing = ProviderPricing {
	name: "xai",
	models: XAI_MODELS,
};

const XAI_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "grok-4.3",
		input_cached: Some(0.2),
		input_normal: 1.25,
		output_normal: 2.5,
		output_reasoning: None,
	},
	ModelPricing {
		name: "grok-build-0.1",
		input_cached: Some(0.2),
		input_normal: 1.0,
		output_normal: 2.0,
		output_reasoning: None,
	},
];
