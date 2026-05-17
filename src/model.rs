#[derive(Debug, Clone, Copy)]
pub struct AiCost {
	pub total: f64,

	pub input_total: f64,
	pub input_normal: f64,
	pub input_cache_read: f64,
	pub input_cache_write: f64,

	pub output_total: f64,
	pub output_normal: f64,
	pub output_reasoning: f64,

	pub input_cache_saving: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ModelPricing {
	pub name: &'static str,
	pub input_cached: Option<f64>,
	pub input_normal: f64,
	pub output_normal: f64,
	pub output_reasoning: Option<f64>,
}

#[derive(Debug)]
pub struct ProviderPricing {
	pub name: &'static str,
	pub models: &'static [ModelPricing],
}
