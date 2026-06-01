use crate::{ModelPricing, ProviderPricing};

pub const MINIMAX: ProviderPricing = ProviderPricing {
	name: "minimax",
	models: MINIMAX_MODELS,
};

const MINIMAX_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "MiniMax-M3",
		input_cached: Some(0.06),
		input_normal: 0.3,
		output_normal: 1.2,
		output_reasoning: None,
	},
	ModelPricing {
		name: "MiniMax-M2.7",
		input_cached: Some(0.06),
		input_normal: 0.3,
		output_normal: 1.2,
		output_reasoning: None,
	},
	ModelPricing {
		name: "MiniMax-M2.7-highspeed",
		input_cached: Some(0.06),
		input_normal: 0.6,
		output_normal: 2.4,
		output_reasoning: None,
	},
];
