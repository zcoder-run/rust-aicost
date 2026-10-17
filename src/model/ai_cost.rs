// region:    --- Types

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

	/// The pricing tier applied, `None` when the base prices were used.
	pub tier: Option<TierInfo>,
}

/// Prices in USD per 1,000,000 tokens.
/// Information about the pricing tier applied to a cost computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierInfo {
	/// The `above_prompt_tokens` threshold of the applied tier.
	pub above_prompt_tokens: u32,
}

// endregion: --- Types
