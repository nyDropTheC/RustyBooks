use worker::*;

use reqwest::{Client, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum ChatCompletionRole {
    System,
    User,
    Assistant
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    role: ChatCompletionRole,
    content: String
}

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32
}

#[derive(Serialize, Deserialize)]
struct BookData {
    author: String, 
    name: String
}

enum RequestType {
    Summarize,
    Similar
}

async fn openai_call(api_key: String, req_type: RequestType, _data: &BookData) {
    let cl = Client::new();

    let messages = match req_type {
        RequestType::Summarize => {
            vec![
                ChatMessage {
                    role: ChatCompletionRole::User,
                    content: "Hi!".to_string()
                }
            ]
        },
        RequestType::Similar => {
            vec![
                ChatMessage {
                    role: ChatCompletionRole::User,
                    content: "Hi!".to_string()
                }
            ]
        }
    };

    let request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        temperature: 1.0f32,
        messages: messages
    };

    console_log!("{}", serde_json::to_string(&request).unwrap());

    let result = cl
        .post("https://api.openai.com/v1/completions")
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await;

    console_log!("{:?}", result);
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .post_async("/summary", |mut req, env| async move {
            let data: BookData = req.json().await?;
            openai_call(env.secret("OPENAI_KEY")?.to_string(), RequestType::Summarize, &data).await;

            Response::ok("lol")
        })
        .post_async("/similar", |mut req, env| async move {
            let data: BookData = req.json().await?;
            openai_call(env.secret("OPENAI_KEY")?.to_string(), RequestType::Similar, &data).await;

            Response::ok("lol")
        })
        .run(req, env)
        .await
        
}
