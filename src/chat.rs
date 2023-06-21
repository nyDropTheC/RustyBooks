use serde::{Serialize, Deserialize};

pub mod chat {
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
}
