// Not for now - Run(for cli): `cargo run --release -- -p "What do you think about Rust lang?" -m ./open_llama_3b-f16.bin`

// Example:
// http://127.0.0.1:8000/
// http://127.0.0.1:8000/chat
// http://127.0.0.1:8000/chat/this is my prompt
// http://127.0.0.1:8000//chat?prompt=this is my prompt

use std::{convert::Infallible, io::Write, net::SocketAddr, path::PathBuf};

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use clap::Parser;
use serde::Deserialize;
use serde_json::json;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    model_path: std::path::PathBuf,

    #[clap(short, long)]
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct Prompt {
    prompt: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Response {
    response: String,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Unable to start server")]
    UnableToStartServer,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router: Router = Router::new()
        .route("/", get(handler_root))
        .route("/chat", get(handler_chat_query))
        .route("/chat/:prompt", get(handler_chat_path));

    // Making a local host
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // ---------- Start the server ---------------
    colour::green_ln!(">> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|_| Error::UnableToStartServer)?;

    Ok(())
}

// Handles the root("/")
async fn handler_root() -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Root");
    Html("<h1>Welcome to chatbot</h1>")
}

// Handles the "/chat?prompt='..'"
async fn handler_chat_query(Query(prompt): Query<Prompt>) -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Handling chat - {:?}", &prompt);

    let llm_response = match prompt.prompt {
        Some(_) => prompt.infer().unwrap(),
        None => {
            return Json(json!({"Suggestion":"To chat do \"/chat?prompt=my prompt\""}));
            // return Html("Do /chat?prompt=my prompt");
        }
    };

    Json(json!({"prompt": prompt.prompt, "response": llm_response.response}))
}

// Handles the "/chat?prompt='..'"
async fn handler_chat_path(Path(prompt): Path<Prompt>) -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Handling chat - {:?}", &prompt);

    let llm_response = match prompt.prompt {
        Some(_) => prompt.infer().unwrap(),
        None => {
            return Json(json!({"Suggestion":"To chat do \"/chat?prompt=my prompt\""}));
            // return Html("Do /chat?prompt=my prompt");
        }
    };

    Json(json!({"prompt": prompt.prompt, "response": llm_response.response}))
}

impl Prompt {
    fn infer(&self) -> anyhow::Result<Response> {
        // What model to use
        let model_architecture = llm::ModelArchitecture::Llama;
        // Path of model binary
        let model_path =
            PathBuf::from("/home/tan/Documents/Projects/llm_task/my_llm/open_llama_3b-f16.bin");
        // Path of tokenizer
        let tokenizer_source = llm::TokenizerSource::Embedded;
        // Prompt to ask
        let prompt = self.prompt.as_deref().unwrap();
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
                llm::InferenceResponse::PromptToken(t)
                | llm::InferenceResponse::InferredToken(t) => {
                    print!("{t}");
                    std::io::stdout().flush().unwrap();

                    Ok(llm::InferenceFeedback::Continue)
                }
                _ => Ok(llm::InferenceFeedback::Continue),
            },
        )?;

        Ok(Response {
            response: result.to_string(),
        })
    }
}
