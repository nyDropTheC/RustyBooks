use worker::*;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct BookData {
    author: String, 
    name: String
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    message: String
}

#[derive(Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>
}

async fn openai_call(api_key: String, data: BookData) {
    let client = Client::new();

    let _ = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .bearer_auth(api_key);
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .post_async("/summary", |req, env| async move {
            let client = Client::new();
            Response::ok("lol")
        })
        .post_async("/similar", |req, env| async move {
            let client = Client::new();
            Response::ok("xd")
        })
        .run(req, env)
        .await
}
