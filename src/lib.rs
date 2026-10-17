// region:    --- Modules

mod cost;
mod data;
mod error;
mod model;

pub use cost::{compute, compute_iden, model_pricing};
pub use error::{Error, Result};
pub use model::{AiCost, ModelPricing, PricingTier, ProviderPricing, TokenPrices};

// endregion: --- Modules
