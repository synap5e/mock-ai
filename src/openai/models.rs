use serde::Deserialize;

#[derive(PartialEq, Clone, Deserialize)]
pub enum ContentType {
    Text,
    ImageUrl,
}

#[derive(Deserialize, Clone)]
pub struct Content {
    pub content_type: ContentType,
    pub text: Option<String>,
    image_url: Option<String>,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum ContentVariant {
    Simple(String),
    Complex(Vec<Content>),
}

#[derive(Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Option<ContentVariant>,
}

#[derive(Deserialize)]
pub struct ChatPayload {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
pub struct EmbeddingPayload {
    pub model: String,
    pub input: EmbeddingInput,
}
