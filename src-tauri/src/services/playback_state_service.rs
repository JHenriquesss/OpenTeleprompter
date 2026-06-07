use crate::domain::errors::AppError;
use crate::domain::playback_state::ScriptPlaybackState;
use crate::persistence::playback_state_repository::PlaybackStateRepository;
use chrono::Utc;

pub struct PlaybackStateService {
    repo: PlaybackStateRepository,
}

impl PlaybackStateService {
    pub fn new(repo: PlaybackStateRepository) -> Self {
        Self { repo }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save(
        &self,
        script_id: String,
        scroll_offset_px: f64,
        speed_multiplier: f64,
        font_size: Option<f64>,
        line_height: Option<f64>,
        mirror_mode: Option<bool>,
        mirror_vertical: Option<bool>,
    ) -> Result<ScriptPlaybackState, AppError> {
        let state = ScriptPlaybackState {
            script_id,
            scroll_offset_px,
            speed_multiplier,
            font_size,
            line_height,
            mirror_mode,
            mirror_vertical,
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.save(&state)
    }

    pub fn load(&self, script_id: String) -> Result<ScriptPlaybackState, AppError> {
        self.repo.load_by_script_id(&script_id)
    }

    pub fn clear(&self, script_id: String) -> Result<(), AppError> {
        // Ignore NotFound — clearing a nonexistent state is fine
        match self.repo.delete(&script_id) {
            Ok(()) | Err(AppError::NotFound(_)) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
