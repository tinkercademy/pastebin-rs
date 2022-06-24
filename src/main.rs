use axum::{routing::{get, post}, response::IntoResponse, Router, Json};

use std::net::SocketAddr;
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::Html;
use tracing::{debug};
use serde::Serialize;
use sqlx::{Connection, SqliteConnection};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    layer::{SubscriberExt},
    util::SubscriberInitExt
};


#[derive(Serialize)]
struct Resp {
    message: String
}


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust_based_pastebin=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(index))
        .route("/v1/paste", post(create_paste))
        .layer(TraceLayer::new_for_http())
        .fallback(not_found.into_service());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(Resp{
            message: "This endpoint was not found".to_string()
        }
        )
    )
}


async fn index() -> impl IntoResponse {
    Html("<h1>API Base</h1>")
}

async fn create_paste(payload: String) -> impl IntoResponse {

    debug!("{}", payload);

    let mut conn = SqliteConnection::connect("sqlite://test.db").await.unwrap();
    sqlx::query("INSERT INTO pastes (content) VALUES (?)")
        .bind(&payload)
        .execute(&mut conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(Resp{
        message: format!("Your paste of {}B was created", payload.len())
    }))
}