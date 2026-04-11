//! E2E tests: voice HTTP API routes (status, intent, pipeline, transcribe).

use std::sync::{Arc, Mutex};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use convergio_voice::tts::TtsEngine;
use tower::ServiceExt;

fn build_app() -> axum::Router {
    let state = convergio_voice::routes::VoiceState {
        tts: Arc::new(Mutex::new(TtsEngine::new())),
    };
    convergio_voice::routes::voice_routes(state)
}

async fn body_json(resp: axum::http::Response<Body>) -> serde_json::Value {
    let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

fn get(uri: &str) -> Request<Body> {
    Request::builder().uri(uri).body(Body::empty()).unwrap()
}

fn post_json(uri: &str, body: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_owned()))
        .unwrap()
}

#[tokio::test]
async fn api_voice_status() {
    let resp = build_app().oneshot(get("/api/voice/status")).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;
    assert_eq!(json["status"], "ok");
    assert!(json["tts_backend"].is_string());
}

#[tokio::test]
async fn api_voice_intent() {
    let req = post_json("/api/voice/intent", r#"{"text":"start voice now"}"#);
    let resp = build_app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;
    assert_eq!(json["intent_type"], "Control");
    assert_eq!(json["command"], "cvg voice start");
}

#[tokio::test]
async fn api_voice_pipeline() {
    let resp = build_app()
        .oneshot(get("/api/voice/pipeline"))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let json = body_json(resp).await;
    assert!(json["tts_backend"].is_string());
    assert_eq!(json["stt_model"], "small");
    assert_eq!(json["wake_word"], "convergio");
    assert_eq!(json["pipeline_state"], "idle");
}

#[tokio::test]
async fn api_transcribe_empty_errors() {
    let resp = build_app()
        .oneshot(post_json("/api/voice/transcribe", "{}"))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn api_transcribe_stub_errors() {
    let req = post_json(
        "/api/voice/transcribe",
        r#"{"samples_i16":[1,2,3],"model":"small"}"#,
    );
    let resp = build_app().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
