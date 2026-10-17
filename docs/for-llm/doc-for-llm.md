# aicost - LLM Documentation

LLM cost calculator for Rust, integrating with `genai` crate types.

## Core API

- `compute(provider: &str, model: &str, usage: &Usage) -> Result<AiCost>`: Calculate cost from strings and `genai::chat::Usage`.
- `compute_iden(iden: &ModelIden, usage: &Usage) -> Result<AiCost>`: Calculate cost from `genai::ModelIden`.
- `model_pricing(iden: &ModelIden) -> Option<ModelPricing>`: Get pricing info for a model.

## Data Structures

### AiCost

```rust
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
    pub tier: Option<TierInfo>,
}
```

### ModelPricing

Prices are in USD per 1,000,000 tokens.
```rust
pub struct ModelPricing {
    pub name: &'static str,
    pub prices: TokenPrices,

    /// Higher price tiers, sorted by ascending `above_prompt_tokens`. Empty for most models.
    /// The whole request is billed at the selected tier (not a marginal rate).
    pub tiers: &'static [PricingTier],
}

/// Tier applies when the total prompt tokens are strictly greater than this value.
pub struct PricingTier {
    pub above_prompt_tokens: u32,
    pub prices: TokenPrices,
}

/// Prices in USD per 1,000,000 tokens.
pub struct TokenPrices {
    pub input_normal: f64,
    pub input_cached: Option<f64>,
    pub input_cache_write: Option<f64>,
    pub output_normal: f64,
    pub output_reasoning: Option<f64>,
}
```

`ModelPricing::prices_for(prompt_tokens)` returns the `TokenPrices` of the highest tier whose `above_prompt_tokens` is exceeded, or the base `prices` when none applies.

`ModelPricing::tier_for(prompt_tokens)` returns the `PricingTier` of that highest tier, or `None` when the base `prices` apply.

### TierInfo

```rust
pub struct TierInfo {
    pub above_prompt_tokens: u32,
}
```

`TierInfo` identifies the applied tier in `AiCost.tier`, and is `None` when the base prices were used.

## Internal Logic

- **Normalization**: Provider `openai_resp` is mapped to `openai`. Model names have prefixes before `::` removed (e.g., `namespace::model` becomes `model`).
- **Matching**: Models are matched using longest prefix matching against the provider's model list.
- **Caching**: 
    - `input_cache_read` uses `usage.prompt_tokens_details.cached_tokens`.
    - `input_cache_write` uses `usage.prompt_tokens_details.cache_creation_tokens` (priced at `input_cache_write`, falling back to 1.25x `input_normal` when absent).
- **Reasoning**: `output_reasoning` uses `usage.completion_tokens_details.reasoning_tokens`.
- **Tiers**: the tier basis is `usage.prompt_tokens`. A tier applies when the prompt tokens are strictly greater than `above_prompt_tokens`. The whole request is billed at the selected tier. `AiCost.tier` is `Some(TierInfo)` when a tier was applied, and `None` for base prices.
- **Rounding**: Costs are rounded to 6 decimal places.

## Supported Providers

OpenAI, Anthropic, Gemini, DeepSeek, Groq, xAI, Fireworks, Together, Zai.
