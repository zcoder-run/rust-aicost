# aicost

A Rust library to compute LLM usage costs based on the [genai](https://crates.io/crates/genai) crate types.

[doc-for-llm.md](docs/for-llm/doc-for-llm.md)

## Features

- Supports multiple providers (OpenAI, Anthropic, Gemini, DeepSeek, Groq, xAI, Fireworks, Together, Zai).
- Handles normal tokens, cached tokens (read/write), and reasoning tokens.
- Provides granular cost breakdown via the `AiCost` struct.
- Integrates with [genai](https://crates.io/crates/genai) via `ModelIden` and `Usage`.

## Usage

```rust
use aicost::compute;
use genai::chat::Usage;

// From a genai::chat::Usage object
let usage = Usage {
    prompt_tokens: Some(1000),
    completion_tokens: Some(500),
    ..Default::default()
};

let cost = compute("openai", "gpt-4o", &usage)?;

println!("Total: ${:.6}", cost.total);
println!("Input Total: ${:.6}", cost.input_total);
println!("Output Total: ${:.6}", cost.output_total);
```

### Using ModelIden

If you are using the `genai` crate, you can compute costs directly using `ModelIden`.

```rust
let cost = aicost::compute_iden(&model_iden, &usage)?;
```

## Structure

The `AiCost` struct provides a detailed breakdown:

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

---

_[This Repo](https://github.com/zcoder-run/rust-aicost)_