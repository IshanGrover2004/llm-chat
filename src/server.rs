use crate::chat::Prompt;
use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;

// Handles the root("/")
async fn handler_root() -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Root");
    Html("<h1>Welcome to Chatbot</h1>
<p><strong>Suggestion</strong>: To initiate a chat, add <code>/chat?prompt=this is prompt</code> or <code>/chat/this is prompt</code> to the path of this site.</p>")
}

// Handles the "/chat?prompt='..'"
async fn handler_chat_query(Query(prompt): Query<Prompt>) -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Handling /chat - {:?}", &prompt);

    let llm_response = match prompt.prompt() {
        Some(_) => prompt.infer().unwrap(),
        None => {
            return Json(
                json!({"Suggestion":"To initiate a chat, add \"/chat?prompt=my prompt\" or \"/chat/my prompt\""}),
            );
            // return Html("Do /chat?prompt=my prompt");
        }
    };

    Json(json!({"prompt": prompt.prompt(), "response": llm_response.response()}))
}

// Handles the "/chat/my prompt.."
async fn handler_chat_path(Path(prompt): Path<Prompt>) -> impl IntoResponse {
    colour::blue_ln!(">> HANDLER - Handling /chat - {:?}", &prompt);

    let llm_response = match prompt.prompt() {
        Some(_) => prompt.infer().unwrap(),
        None => {
            return Json(json!({"Suggestion":"To chat do \"/chat?prompt=my prompt\""}));
            // return Html("Do /chat?prompt=my prompt");
        }
    };

    Json(json!({"prompt": prompt.prompt(), "response": llm_response.response()}))
}

pub async fn start_server() -> anyhow::Result<()> {
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
        .await?;

    Ok(())
}
