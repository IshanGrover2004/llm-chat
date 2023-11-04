// Not for now - Run(for cli): `cargo run --release -- -p "What do you think about Rust lang?" -m ./open_llama_3b-f16.bin`

use std::{convert::Infallible, io::Write, net::SocketAddr, path::PathBuf};

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    model_path: std::path::PathBuf,

    #[clap(short, long)]
    prompt: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Unable to start server")]
    UnableToStartServer,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router: Router = Router::new().route("/", get(handler_root));

    // Making a local host
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // ---------- Start the server ---------------
    colour::green_ln!(">> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|_| Error::UnableToStartServer)?;

    Ok(())
}

// Hanle the root("/") path
async fn handler_root() -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Root");
    Html("<h1>Welcome to chatbot</h1>")
}

fn infer(model_path: PathBuf, prompt: impl AsRef<str>) -> anyhow::Result<()> {
    // What model to use
    let model_architecture = llm::ModelArchitecture::Llama;
    // Path of model binary
    let model_path = model_path;
    // Path of tokenizer
    let tokenizer_source = llm::TokenizerSource::Embedded;
    // Prompt to ask
    let prompt = prompt.as_ref();
    // Time at which program is executed
    let now_time = std::time::Instant::now();

    // Loading model with necessary details
    let model = llm::load_dynamic(
        Some(model_architecture),
        &model_path,
        tokenizer_source,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    println!(
        "Model fully loaded! Elapsed: {}ms",
        now_time.elapsed().as_millis()
    );

    // Starting session
    let mut session = model.start_session(Default::default());

    let result = session.infer::<Infallible>(
        // Model to use
        model.as_ref(),
        &mut rand::thread_rng(),
        // Request for prompt
        &llm::InferenceRequest {
            prompt: (prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        // Output request
        &mut Default::default(),
        // Inference response
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();

                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match result {
        Ok(result) => println!("Inference stats: \n {result}"),
        Err(err) => eprintln!("{err}"),
    }

    Ok(())
}
