use axum::{extract::State, routing::post, Json, Router};
use serde_json::Value;

use super::models::{ChatPayload, EmbeddingPayload};
use super::services::{generate_chat_completion, generate_embeddings};

use crate::server::{AppError, AppState};

#[axum_macros::debug_handler]
async fn create_embeddings_handler(
    State(state): State<AppState>,
    Json(payload): Json<EmbeddingPayload>,
) -> Json<Value> {
    let value = generate_embeddings(state.embedding_size, payload).await;

    Json(value)
}

#[axum_macros::debug_handler]
async fn create_chat_completion_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatPayload>,
) -> Result<Json<Value>, AppError> {
    let value = generate_chat_completion(payload, state.custom_responses).await?;
    Ok(Json(value))
}

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/chat/completions", post(create_chat_completion_handler))
        .route(
            "/deployments/{path}/chat/completions",
            post(create_chat_completion_handler),
        )
        .route("/embeddings", post(create_embeddings_handler))
        .route(
            "/deployments/{path}/embeddings",
            post(create_embeddings_handler),
        )
}
