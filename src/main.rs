use axum::{routing::{get, post}, response::IntoResponse, Router, Json, Extension};

use std::net::SocketAddr;
use axum::extract::{Path};
use dotenv::dotenv;
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::Html;
use tracing::{debug};
use serde::Serialize;
use sqlx::Row;
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    layer::{SubscriberExt},
    util::SubscriberInitExt
};
use rust_based_pastebin::ApiContext;


#[derive(Serialize)]
struct Resp {
    message: String
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust_based_pastebin=debug,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://:memory:".into());

    let db = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url).await.unwrap_or_else(|e| {
            panic!("Failed to connect to database: {:?}", e);
        });

    let app = Router::new()
        .route("/", get(index))
        .route("/v1/paste", post(create_paste))
        .route("/v1/paste/:id", get(view_paste))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(ApiContext {
            db
        }))
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

async fn view_paste(Path(id): Path<u32>, ctx: Extension<ApiContext>) -> (StatusCode, String) {
    let mut conn = ctx.db.begin().await.unwrap();

    let row = sqlx::query("SELECT content FROM pastes WHERE id = $1")
        .bind(id)
        .fetch_one(&mut conn)
        .await;
    match row {
        Ok(row) => {
            let content: String= row.try_get("content").unwrap();
            (StatusCode::OK, content)
        }
        Err(_) => (StatusCode::NOT_FOUND, "Paste not found".to_string())
    }
}

async fn create_paste(payload: String, ctx: Extension<ApiContext>) -> impl IntoResponse {
    let mut conn = ctx.db.begin().await.unwrap();

    debug!("Payload size: {}", payload.len());

    sqlx::query("INSERT INTO pastes (content) VALUES (?)")
        .bind(&payload)
        .execute(&mut conn)
        .await
        .unwrap();
    conn.commit().await.unwrap();

    (StatusCode::OK, Json(Resp{
        message: format!("Your paste of {}B was created", payload.len())
    }))
}