use crate::domain::playback_state::ScriptPlaybackState;
use crate::services::playback_state_service::PlaybackStateService;
use tauri::State;

pub struct PlaybackStateCommandHandler {
    pub service: PlaybackStateService,
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn save_playback_state(
    handler: State<PlaybackStateCommandHandler>,
    script_id: String,
    scroll_offset_px: f64,
    speed_multiplier: f64,
    font_size: Option<f64>,
    line_height: Option<f64>,
    mirror_mode: Option<bool>,
    mirror_vertical: Option<bool>,
) -> Result<ScriptPlaybackState, String> {
    handler
        .service
        .save(
            script_id,
            scroll_offset_px,
            speed_multiplier,
            font_size,
            line_height,
            mirror_mode,
            mirror_vertical,
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_playback_state(
    handler: State<PlaybackStateCommandHandler>,
    script_id: String,
) -> Result<Option<ScriptPlaybackState>, String> {
    match handler.service.load(script_id) {
        Ok(state) => Ok(Some(state)),
        Err(e) if e.to_string().contains("not found") => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn clear_playback_state(
    handler: State<PlaybackStateCommandHandler>,
    script_id: String,
) -> Result<(), String> {
    handler.service.clear(script_id).map_err(|e| e.to_string())
}
