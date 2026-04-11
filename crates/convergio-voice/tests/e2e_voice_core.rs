//! E2E tests: intent extraction, wake word, audio utilities, whisper,
//! TTS engine, and voice config.

use convergio_voice::audio_util;
use convergio_voice::intent::{extract_intent, IntentType};
use convergio_voice::tts::TtsEngine;
use convergio_voice::wake_word::WakeWordDetector;
use convergio_voice::whisper::WhisperEngine;
use convergio_voice::{SpeechSegment, VoiceConfig};

// ── intent extraction ────────────────────────────────────────────────────────

#[test]
fn intent_control_start_voice() {
    let i = extract_intent("please start voice now").unwrap();
    assert_eq!(i.intent_type, IntentType::Control);
    assert_eq!(i.command.as_deref(), Some("cvg voice start"));
    assert!(i.confidence > 0.8);
}

#[test]
fn intent_control_stop() {
    let i = extract_intent("stop everything").unwrap();
    assert_eq!(i.intent_type, IntentType::Control);
    assert_eq!(i.command.as_deref(), Some("cvg voice stop"));
}

#[test]
fn intent_control_status() {
    let i = extract_intent("show status").unwrap();
    assert_eq!(i.intent_type, IntentType::Control);
    assert_eq!(i.command.as_deref(), Some("cvg voice status"));
}

#[test]
fn intent_navigation() {
    let i = extract_intent("go to mesh").unwrap();
    assert_eq!(i.intent_type, IntentType::Navigation);
    assert_eq!(i.command.as_deref(), Some("navigate:mesh"));
}

#[test]
fn intent_command_plan_list() {
    let i = extract_intent("show me the plan list").unwrap();
    assert_eq!(i.intent_type, IntentType::Command);
    assert_eq!(i.command.as_deref(), Some("cvg plan list"));
}

#[test]
fn intent_command_agents() {
    let i = extract_intent("who are the agents listed").unwrap();
    assert_eq!(i.intent_type, IntentType::Command);
    assert_eq!(i.command.as_deref(), Some("cvg who agents"));
}

#[test]
fn intent_query() {
    let i = extract_intent("what is the current build progress").unwrap();
    assert_eq!(i.intent_type, IntentType::Query);
    assert!(i.command.is_none());
}

#[test]
fn intent_ambiguous() {
    let i = extract_intent("do something").unwrap();
    assert_eq!(i.intent_type, IntentType::Ambiguous);
    assert!(i.confidence < 0.5);
}

// ── wake word ────────────────────────────────────────────────────────────────

#[test]
fn wake_word_detect_and_reset() {
    let mut d = WakeWordDetector::new("convergio", 0.5, "small");
    assert!(!d.is_detected());
    assert!(d.check_text("Hey Convergio, do something").unwrap());
    assert!(d.is_detected());
    d.reset();
    assert!(!d.is_detected());
}

#[test]
fn wake_word_case_insensitive() {
    let mut d = WakeWordDetector::new("Convergio", 0.5, "small");
    assert!(d.check_text("CONVERGIO help").unwrap());
}

#[test]
fn wake_word_no_match() {
    let mut d = WakeWordDetector::new("convergio", 0.5, "small");
    assert!(!d.check_text("hello world").unwrap());
}

// ── audio utilities ──────────────────────────────────────────────────────────

#[test]
fn stereo_to_mono() {
    let mono = audio_util::stereo_to_mono(&[100i16, 200, -100, -200, 0, 0]);
    assert_eq!(mono, vec![150, -150, 0]);
}

#[test]
fn resample_identity() {
    let s = vec![1i16, 2, 3, 4, 5];
    assert_eq!(audio_util::resample(&s, 16000, 16000), s);
}

#[test]
fn resample_downsamples() {
    let s: Vec<i16> = (0..100).collect();
    let r = audio_util::resample(&s, 44100, 16000);
    assert!(r.len() < s.len() && !r.is_empty());
}

#[test]
fn resample_empty() {
    assert!(audio_util::resample(&[], 44100, 16000).is_empty());
}

// ── whisper engine (stub) ────────────────────────────────────────────────────

#[test]
fn whisper_empty_segment_error() {
    let e = WhisperEngine::new("small", true);
    let seg = SpeechSegment {
        start_ms: 0,
        end_ms: 0,
        samples: vec![],
    };
    assert!(e
        .transcribe(&seg)
        .unwrap_err()
        .to_string()
        .contains("empty"));
}

#[test]
fn whisper_local_not_available() {
    let e = WhisperEngine::new("small", true);
    let seg = SpeechSegment {
        start_ms: 0,
        end_ms: 1000,
        samples: vec![1, 2, 3],
    };
    assert!(e
        .transcribe(&seg)
        .unwrap_err()
        .to_string()
        .contains("feature flag"));
}

#[test]
fn whisper_api_not_implemented() {
    let e = WhisperEngine::new("small", false);
    let seg = SpeechSegment {
        start_ms: 0,
        end_ms: 1000,
        samples: vec![1, 2, 3],
    };
    assert!(e
        .transcribe(&seg)
        .unwrap_err()
        .to_string()
        .contains("not yet"));
}

#[test]
fn whisper_model_path() {
    assert!(WhisperEngine::new("small", true)
        .model_path()
        .contains("ggml-small.bin"));
}

// ── TTS engine ───────────────────────────────────────────────────────────────

#[test]
fn tts_selects_backend() {
    let e = TtsEngine::new();
    let name = e.backend().display_name();
    assert!(
        ["Voxtral Mini MLX", "Qwen3 TTS Vivian", "macOS Say"].contains(&name),
        "unexpected: {name}"
    );
}

#[test]
fn tts_has_model_name() {
    let e = TtsEngine::new();
    assert!(!e.model_name.is_empty());
    assert!(e.loaded);
}

// ── voice config ─────────────────────────────────────────────────────────────

#[test]
fn voice_config_defaults() {
    let cfg = VoiceConfig::default();
    assert_eq!(cfg.wake_word, "convergio");
    assert_eq!(cfg.whisper_model, "small");
    assert!(cfg.prefer_local);
}
