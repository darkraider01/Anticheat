use axum_extra::extract::cookie::Key;
use axum::extract::FromRef;

pub mod config; // Declare the config submodule

#[derive(Clone)]
pub struct AppState {
    pub cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.cookie_key.clone()
    }
}