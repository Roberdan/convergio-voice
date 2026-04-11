//! HTTP API routes for convergio-voice.

use axum::Router;

/// Returns the router for this crate's API endpoints.
pub fn routes() -> Router {
    Router::new()
    // .route("/api/voice/health", get(health))
}
