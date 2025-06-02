# AI Integration

Zed integrates large language models through crates like [`open_ai`](../../crates/open_ai).
The crate defines request structures and helpers to communicate with the OpenAI API:

```rust
pub const OPEN_AI_API_URL: &str = "https://api.openai.com/v1";
```

Models are enumerated and provide metadata like maximum token counts.
See [`open_ai.rs`](../../crates/open_ai/src/open_ai.rs) for full details.

When building your own app, create a client module that wraps these APIs and expose them via async functions.
