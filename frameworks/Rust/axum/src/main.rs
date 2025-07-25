mod common;
mod server;

use axum::{response::IntoResponse, routing::get, Router};
use common::models::Message;
use dotenv::dotenv;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(not(feature = "simd-json"))]
use axum::Json;
#[cfg(feature = "simd-json")]
use common::simd_json::Json;

const HELLO_WORLD: &str = "Hello, World!";

/// Return a plaintext static string.
#[inline(always)]
pub async fn plaintext() -> &'static str {
    &HELLO_WORLD
}

/// Return a JSON message.
#[inline(always)]
pub async fn json() -> impl IntoResponse {
    let message = Message {
        message: HELLO_WORLD,
    };

    Json(message)
}

fn main() {
    dotenv().ok();
    server::start_tokio(serve_app)
}

async fn serve_app() {

    let app = Router::new()
        .route("/plaintext", get(plaintext))
        .route("/json", get(json));

    server::serve_hyper(app, Some(8000)).await
}