// Example:
// http://127.0.0.1:8000/
// http://127.0.0.1:8000/chat
// http://127.0.0.1:8000/chat/this is my prompt
// http://127.0.0.1:8000//chat?prompt=this is my prompt

/// Module for LLM models
pub mod chat;
/// Module for axum server
pub mod server;

use server::start_server;

// TODO: (using CLI) `cargo run --release -- -m ./open_llama_3b-f16.bin`

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let model = load machine()
    start_server().await?;

    Ok(())
}
