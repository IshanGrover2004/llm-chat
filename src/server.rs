use std::net::SocketAddr;

use crate::chat::Prompt;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;

/// Custom server errors
#[derive(Debug, thiserror::Error)]
enum ServerError {
    #[error("Unable to start server due to: {0}")]
    UnableToStartServer(String),
}

/// Starts the server and handles incoming requests.
pub async fn start_server() -> anyhow::Result<()> {
    let router: Router = Router::new()
        // Defines a route for the root URL ("/")
        .route("/", get(handler_root))
        // Defines a route for the "/chat" URL
        .route("/chat", get(handler_chat_query))
        // Defines a route with a dynamic segment "/chat/:prompt"
        .route("/chat/:prompt", get(handler_chat_path));

    // Representing the IP address and port for the server.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // ---------- Start the server ---------------
    colour::green_ln!(">> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(|e| ServerError::UnableToStartServer(e.to_string()))?;

    Ok(())
}

/// Handles the root("/")
async fn handler_root() -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Root");
    Html("<h1>Welcome to Chatbot</h1>
<p><strong>Suggestion</strong>: To initiate a chat, add <code>/chat?prompt=this is prompt</code> or <code>/chat/this is prompt</code> to the path of this site.</p>")
}

/// Handles the "/chat?prompt='..'"
async fn handler_chat_query(Query(prompt): Query<Prompt>) -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Handling /chat - {:?}", &prompt);

    // Check for prompt
    match prompt.prompt() {
        // Create LLM response
        Some(_) => {
            let llm_response = prompt.infer().unwrap();
            Json(json!({"prompt": prompt.prompt(), "response": llm_response.response()}))
        }
        // Give instruction of "how to chat?"
        None => Json(
            json!({"Suggestion":"To initiate a chat, add \"/chat?prompt=my prompt\" or \"/chat/my prompt\""}),
        ),
    }
}

/// Handles the "/chat/my prompt.."
async fn handler_chat_path(Path(prompt): Path<Prompt>) -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Handling /chat - {:?}", &prompt);

    // Check for prompt
    match prompt.prompt() {
        // Create LLM response
        Some(_) => {
            let llm_response = prompt.infer().expect("Failed to perform inference");
            Json(json!({"prompt": prompt.prompt(), "response": llm_response.response()}))
        }
        // Give instruction of "how to chat?"
        None => Json(
            json!({"Suggestion":"To initiate a chat, add \"/chat?prompt=my prompt\" or \"/chat/my prompt\""}),
        ),
    }
}
