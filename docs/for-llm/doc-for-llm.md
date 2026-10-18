# aicost - LLM Documentation

LLM cost calculator for Rust, integrating with `genai` crate types.

## Core API

- `compute(provider: &str, model: &str, usage: &Usage) -> Result<AiCost>`: Calculate cost from strings and `genai::chat::Usage`.
- `compute_iden(iden: &ModelIden, usage: &Usage) -> Result<AiCost>`: Calculate cost from `genai::ModelIden`.
- `model_pricing(iden: &ModelIden) -> Option<ModelPricing>`: Get pricing info for a model.

### Errors

- `Result<T>`: Alias for `std::result::Result<T, Error>`.
- `Error::ModelNotFound(provider, model)`: Returned by `compute` and `compute_iden` when no pricing entry matches.

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

`ModelPricing::tier_index_for(prompt_tokens)` returns the index of that highest tier in `tiers`, or `None` when the base `prices` apply.

### TierInfo

```rust
pub struct TierInfo {
    pub tier_index: usize,
}
```

`TierInfo` identifies the applied tier in `AiCost.tier`. `tier_index` is the position of that tier in `ModelPricing.tiers`. `AiCost.tier` is `None` when the base prices were used.

### ProviderPricing

Groups the model pricing entries of one provider. Provider tables are internal data, not reached through the public API.

```rust
pub struct ProviderPricing {
    pub name: &'static str,
    pub models: &'static [ModelPricing],
}
```

## Internal Logic

- **Normalization**: Provider `openai_resp` is mapped to `openai` and `gemini_ix` to `gemini`. Model names have prefixes before `::` removed (e.g., `namespace::model` becomes `model`).
- **Matching**: Models are matched using longest prefix matching against the provider's model list.
- **Caching**: 
    - `input_cache_read` uses `usage.prompt_tokens_details.cached_tokens` (priced at `input_cached`, falling back to `input_normal` when absent).
    - `input_cache_write` uses `usage.prompt_tokens_details.cache_creation_tokens` (priced at `input_cache_write`, falling back to 1.25x `input_normal` when absent).
- **Reasoning**: `output_reasoning` uses `usage.completion_tokens_details.reasoning_tokens` (priced at `output_reasoning`, falling back to `output_normal` when absent).
- **Tiers**: the tier basis is `usage.prompt_tokens`. A tier applies when the prompt tokens are strictly greater than `above_prompt_tokens`. The whole request is billed at the selected tier. `AiCost.tier` is `Some(TierInfo)` with `tier_index` set to the applied tier's position in `tiers` when a tier was applied, and `None` for base prices.
- **Token split**: `prompt_tokens` is the total prompt size. `input_normal` uses `prompt_tokens - cached_tokens - cache_creation_tokens`. Without `prompt_tokens_details`, all prompt tokens are normal. Missing usage counts default to `0`.
- **Output split**: `output_normal` uses `completion_tokens - reasoning_tokens`.
- **Cache saving**: `input_cache_saving` is `cached_tokens * (input_normal - cached price) / 1M`, minus the `input_cache_write` cost. It is `0` when there are no cache tokens, and it can be negative when cache writes outweigh read savings.
- **Rounding**: Each component (`input_normal`, `input_cache_read`, `input_cache_write`, `output_normal`, `output_reasoning`) is rounded to 6 decimal places. Subtotals and `total` are sums of the rounded components, also rounded to 6 decimal places.

## Supported Providers

OpenAI, Anthropic, Gemini, DeepSeek, Groq, xAI, Fireworks, Together, Zai.
