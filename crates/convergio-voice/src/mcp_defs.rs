//! MCP tool definitions for the voice extension.

use convergio_types::extension::McpToolDef;
use serde_json::json;

pub fn voice_tools() -> Vec<McpToolDef> {
    vec![
        McpToolDef {
            name: "cvg_voice_status".into(),
            description: "Get voice pipeline status.".into(),
            method: "GET".into(),
            path: "/api/voice/status".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "community".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_voice_speak".into(),
            description: "Generate speech from text.".into(),
            method: "POST".into(),
            path: "/api/voice/speak".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "text": {"type": "string", "description": "Text to speak"}
                },
                "required": ["text"]
            }),
            min_ring: "trusted".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_voice_intent".into(),
            description: "Detect intent from voice input.".into(),
            method: "POST".into(),
            path: "/api/voice/intent".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "audio_url": {"type": "string"}
                },
                "required": ["audio_url"]
            }),
            min_ring: "trusted".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_voice_transcribe".into(),
            description: "Transcribe audio to text.".into(),
            method: "POST".into(),
            path: "/api/voice/transcribe".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "audio_url": {"type": "string"}
                },
                "required": ["audio_url"]
            }),
            min_ring: "trusted".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_voice_pipeline".into(),
            description: "Get voice pipeline configuration.".into(),
            method: "GET".into(),
            path: "/api/voice/pipeline".into(),
            input_schema: json!({"type": "object", "properties": {}}),
            min_ring: "community".into(),
            path_params: vec![],
        },
    ]
}
