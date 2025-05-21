use serde_json::{json, Value};

use crate::models::{CustomResponses, FunctionOutputs, ResponseType};

use super::models::{
    ChatPayload, ContentType, ContentVariant, EmbeddingInput, EmbeddingPayload, Role,
};

fn json_response(model: String, content: Option<String>, tool_calls: Option<Vec<Value>>) -> Value {
    json!({
        "id": format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        "object": "chat.completion",
        "created": std::time::SystemTime::now(),
        "model": model,
        "system_fingerprint": "mock",
        "choices": [
            {
                "index": 0,
                "logprobs": Value::Null,
                "finish_reason": "stop",
                "message": {
                    "role": "assistant",
                    "content": content,
                    "tool_calls": tool_calls
                },
            }
        ],
        "usage": {
            "prompt_tokens": 0,
            "completion_tokens": 0,
            "total_tokens": 0,
            "completion_tokens_details": {"reasoning_tokens": 0},
        }
    })
}

fn into_openai_format(
    response_type: ResponseType,
    response: String,
) -> (Option<String>, Option<Vec<Value>>) {
    match response_type {
        ResponseType::Text => (Some(response), None),
        ResponseType::Function => {
            let tool_calls = serde_json::from_str::<FunctionOutputs>(&response)
                .expect("Already validated to be correct")
                .iter()
                .map(|item| {
                    json!({
                        "id": uuid::Uuid::new_v4().to_string(),
                        "type": "function",
                        "function": {
                            "name": item.name,
                            "arguments": item.arguments
                        }
                    })
                })
                .collect::<Vec<_>>();

            (None, Some(tool_calls))
        }
    }
}

pub async fn generate_chat_completion(
    payload: ChatPayload,
    custom_responses: CustomResponses,
) -> anyhow::Result<Value> {
    let mut response_type = ResponseType::Text;

    payload
        .messages
        .iter()
        .rev()
        .find(|message| message.role == Role::User)
        .map(|message| message.content.clone())
        .flatten()
        .ok_or_else(|| anyhow::anyhow!("Content from last message cannot be None"))
        .and_then(|content| match content {
            ContentVariant::Simple(string) => Ok(string),
            ContentVariant::Complex(list) => list
                .iter()
                .rev()
                .find(|item| item.content_type == ContentType::Text)
                .map(|item| item.text.clone())
                .flatten()
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Content array must include at least one object with 'type' = 'text'",
                    )
                }),
        })
        .map(|trigger| {
            custom_responses
                .search_matching(&trigger)
                .map_or(trigger, |item| {
                    response_type = item.response_type;
                    item.response
                })
        })
        .map(|response| {
            let (content, tool_calls) = into_openai_format(response_type, response);

            if payload.stream.is_some_and(|value| value) {
                todo!()
            } else {
                json_response(payload.model, content, tool_calls)
            }
        })
}

pub async fn generate_embeddings(embedding_size: usize, payload: EmbeddingPayload) -> Value {
    let input_list = match payload.input {
        EmbeddingInput::Single(string) => vec![string],
        EmbeddingInput::Multiple(vec) => vec,
    };

    json!({
        "object": "list",
        "data": input_list.iter().enumerate().map(|(index, _)| {
            json!({
                "object": "embedding",
                "embedding": (0..embedding_size).map(|_| rand::random::<f32>() * 2.0 - 1.0).collect::<Vec<f32>>(),
                "index": index
            })
        }).collect::<Vec<_>>(),
        "model": payload.model,
        "usage": {
            "prompt_tokens": 0,
            "total_tokens": 0
        }
    })
}
