use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Script {
    id: String,
    title: String,
    content: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppSettings {
    font_size: f64,
    line_height: f64,
    text_width: f64,
    scroll_speed: f64,
    mirror_mode: bool,
    theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            font_size: 32.0,
            line_height: 1.8,
            text_width: 60.0,
            scroll_speed: 1.0,
            mirror_mode: false,
            theme: "Dark".to_string(),
        }
    }
}

#[test]
fn script_can_be_created() {
    let script = Script {
        id: "test-id".into(),
        title: "Test Script".into(),
        content: "Hello world".into(),
        created_at: "2026-01-01T00:00:00Z".into(),
        updated_at: "2026-01-01T00:00:00Z".into(),
    };
    assert_eq!(script.title, "Test Script");
    assert_eq!(script.content, "Hello world");
}

#[test]
fn settings_defaults_are_reasonable() {
    let settings = AppSettings::default();
    assert!(settings.font_size >= 12.0);
    assert!(settings.line_height >= 1.0);
    assert!(settings.text_width >= 20.0);
    assert!(settings.scroll_speed > 0.0);
}

#[test]
fn settings_can_be_serialized() {
    let settings = AppSettings::default();
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("font_size"));
    assert!(json.contains("mirror_mode"));
}

#[test]
fn settings_can_be_deserialized() {
    let json = r#"{
        "font_size": 40.0,
        "line_height": 2.0,
        "text_width": 50.0,
        "scroll_speed": 1.5,
        "mirror_mode": true,
        "theme": "Dark"
    }"#;
    let settings: AppSettings = serde_json::from_str(json).unwrap();
    assert_eq!(settings.font_size, 40.0);
    assert!(settings.mirror_mode);
}

#[test]
fn script_can_be_serialized_roundtrip() {
    let script = Script {
        id: "abc-123".into(),
        title: "My Speech".into(),
        content: "Hello, everyone!".into(),
        created_at: "2026-06-01T12:00:00Z".into(),
        updated_at: "2026-06-01T12:00:00Z".into(),
    };
    let json = serde_json::to_string(&script).unwrap();
    let deserialized: Script = serde_json::from_str(&json).unwrap();
    assert_eq!(script.id, deserialized.id);
    assert_eq!(script.title, deserialized.title);
    assert_eq!(script.content, deserialized.content);
}
