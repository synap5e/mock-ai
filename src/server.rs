use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use super::cli::Cli;
use super::models::CustomResponses;

#[derive(Debug, Clone)]
pub struct AppState {
    pub embedding_size: usize,
    pub custom_responses: CustomResponses,
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    let state = AppState {
        embedding_size: std::env::var("EMBEDDING_SIZE")
            .unwrap_or_else(|_| "1536".to_string())
            .parse()
            .unwrap(),
        custom_responses: CustomResponses::from_file(cli.config)?,
    };

    let openai_router = crate::openai::create_router();

    let app = Router::new()
        .nest("/openai", openai_router)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C handler");
        })
        .await
        .unwrap();

    Ok(())
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}
