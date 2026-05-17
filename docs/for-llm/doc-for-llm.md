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
}
```

### ModelPricing

Prices are in USD per 1,000,000 tokens.
```rust
pub struct ModelPricing {
    pub name: &'static str,
    pub input_cached: Option<f64>,
    pub input_normal: f64,
    pub output_normal: f64,
    pub output_reasoning: Option<f64>,
}
```

## Internal Logic

- **Normalization**: Provider `openai_resp` is mapped to `openai`. Model names have prefixes before `::` removed (e.g., `namespace::model` becomes `model`).
- **Matching**: Models are matched using longest prefix matching against the provider's model list.
- **Caching**: 
    - `input_cache_read` uses `usage.prompt_tokens_details.cached_tokens`.
    - `input_cache_write` uses `usage.prompt_tokens_details.cache_creation_tokens` (costed at 1.25x `input_normal`).
- **Reasoning**: `output_reasoning` uses `usage.completion_tokens_details.reasoning_tokens`.
- **Rounding**: Costs are rounded to 6 decimal places.

## Supported Providers

OpenAI, Anthropic, Gemini, DeepSeek, Groq, xAI, Fireworks, Together, Zai.
