use worker::*;

use reqwest::{Client, Result as ReqResult, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionRole {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    role: ChatCompletionRole,
    content: String,
}

// I should probably implement functions - it'd work better
#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize)]
struct BookData {
    author: String,
    name: String,
}

enum RequestType {
    Summarize,
    Similar,
}

async fn openai_call(
    env: &RouteContext<()>,
    req_type: RequestType,
    data: &BookData,
) -> ReqResult<String> {
    let BookData { author, name } = data;

    let messages = match req_type {
        RequestType::Summarize => {
            vec![
                ChatMessage {
                    role: ChatCompletionRole::System,
                    content: "You are tasked with summarizing the contents of literary works with an intended result of making the summary\'s reader get interested in reading said work.".to_string()
                },

                ChatMessage {
                    role: ChatCompletionRole::User,
                    content: format!("Summarize {} by the author(s) {} in accordance to the given instructions. Space out paragraphs with two newlines. Be engaging and enticing.", name, author)
                }
            ]
        }
        RequestType::Similar => {
            vec![
                ChatMessage {
                    role: ChatCompletionRole::System,
                    content: "You are tasked with suggesting similar works to the one the user provides. Include the name of the recommended work, the recommended work\'s author and a short, but engaging summary of the work that entices the intended reader to read the work.".to_string()
                },

                ChatMessage {
                    role: ChatCompletionRole::User,
                    content: "Suggest me similar works to Pact by the author(s) Wildbow in accordance to the given instructions.
                    Output the data as JSON, with the format being an array of objects with the schema of {name: \"The work's name\", author: \"The work's author\", description: \"A short, but engaging description of the work in question that entices the summary's reader to read the work\"}. Do not output anything but JSON. Do not beautify the JSON.".to_string()
                },

                ChatMessage {
                    role: ChatCompletionRole::Assistant,
                    content: "[{\"name\":\"Worm\",\"author\":\"Wildbow\",\"description\":\"Worm is a web serial about a teenage girl with the power to control bugs and defend her city from a variety of threats.\"},{\"name\":\"Twig\",\"author\":\"Wildbow\",\"description\":\"Twig is a web serial about a group of children experiments in an alternate world where mad science is the norm.\"},{\"name\":\"Mother of Learning\",\"author\":\"Domagoj Kurmaic\",\"description\":\"Mother of Learning is a web novel about a young mage stuck in a time loop, trying to solve a deadly magical mystery.\"}]".to_string()
                },

                ChatMessage {
                    role: ChatCompletionRole::User,
                    content: format!("Suggest me similar works to {} by the author(s) {} in accordance to the given instructions.
                    Output the data as JSON, with the format being an array of objects with the schema of {{name: \"The work's name\", author: \"The work's author\", description: \"A short, but engaging description of the work in question that entices the summary's reader to read the work\"}}. Do not output anything but JSON. Do not beautify the JSON.", name, author)
                }
            ] // See, *this* is where functions would clutch shit
        }
    };

    let request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: messages,
    };

    let result = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .json(&request)
        .bearer_auth(env.secret("OPENAI_KEY").unwrap().to_string()) // .unwrap() is probably safe here?
        .send()
        .await?;

    Ok(result.text().await?)
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // responders could probably be made better with a fn returning proper closure for each responder
    // but I don't care

    // this is retarded what the fuck
    // I don't get why I need the worker URL

    let result = env
        .service("auth")?
        .fetch_request(Request::new_with_init(
            &env.secret("VERIFIER_URI")?.to_string(),
            RequestInit::default()
                .with_method(Method::Get)
                .with_headers(req.headers().clone()),
        )?)
        .await?;

    if result.status_code() != StatusCode::OK.as_u16() {
        return Response::error("", result.status_code());
    }

    Router::new()
        .post_async("/summary", |mut req, env| async move {
            if let Ok(resp) = openai_call(&env, RequestType::Summarize, &req.json().await?).await {
                let mut headers = Headers::new();

                headers.append("Content-Type", "application/json")?;

                Ok(Response::ok(resp)?.with_headers(headers))
            } else {
                Response::error("", 400)
            }
        })
        .post_async("/similar", |mut req, env| async move {
            if let Ok(resp) = openai_call(&env, RequestType::Similar, &req.json().await?).await {
                let mut headers = Headers::new();

                headers.append("Content-Type", "application/json")?;

                Ok(Response::ok(resp)?.with_headers(headers))
            } else {
                Response::error("", 400)
            }
        })
        .run(req, env)
        .await
}
