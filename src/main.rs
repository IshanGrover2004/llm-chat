// Not for now - Run(for cli): `cargo run --release -- -p "What do you think about Rust lang?" -m ./open_llama_3b-f16.bin`

// Example:
// http://127.0.0.1:8000/
// http://127.0.0.1:8000/chat
// http://127.0.0.1:8000/chat/this is my prompt
// http://127.0.0.1:8000//chat?prompt=this is my prompt

// Module for LLM models
pub mod chat;
// Module for axum server
pub mod server;

use clap::Parser;
use server::start_server;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    model_path: std::path::PathBuf,

    #[clap(short, long)]
    prompt: String,
}

// #[derive(Debug, thiserror::Error)]
// enum Error {
//     #[error("Unable to start server")]
//     UnableToStartServer,
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_server().await?;

    Ok(())
}
