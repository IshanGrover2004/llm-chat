use std::net::SocketAddr;

use crate::chat::Prompt;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
enum ServerError {
    #[error("Unable to start server due to: {0}")]
    UnableToStartServer(#[from] std::io::Error),
}

/// Starts the server and handles incoming requests.
pub async fn start_server() -> anyhow::Result<()> {
    let router: Router = Router::new()
        .route("/", get(handle_root))
        .route("/chat", get(handle_chat_query))
        .route("/chat/:prompt", get(handle_chat_path));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // ---------- Start the server ---------------
    colour::green_ln!(">> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

/// Handles the root("/")
async fn handle_root() -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Root");
    Json(json!("Ok"))
}

/// Handles the "/chat?prompt='..'"
async fn handle_chat_query(Query(prompt): Query<Prompt>) -> impl IntoResponse {
    let mut prompt = prompt;
    colour::blue_ln!(">> HANDLER - Handling /chat - {:?}", &prompt);
    prompt.generate_reply()
}

/// Handles the "/chat/my prompt.."
async fn handle_chat_path(Path(prompt): Path<Prompt>) -> impl IntoResponse {
    let mut prompt = prompt;
    colour::blue_ln!(">> HANDLER - Handling /chat - {:?}", &prompt);
    prompt.generate_reply()
}
