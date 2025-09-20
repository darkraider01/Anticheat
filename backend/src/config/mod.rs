use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
#[derive(Clone)]
pub struct AppState {
cookie_key: Key, // Made private
pub jwt_secret: String,
pub api_key_prefix: String,
}
impl AppState {
pub fn new(cookie_key: Key, jwt_secret: String, api_key_prefix: String) -> Self {
Self {
cookie_key,
jwt_secret,
api_key_prefix,
}
}
pub fn cookie_key(&self) -> &Key {
&self.cookie_key
}
}
impl FromRef<AppState> for Key {
fn from_ref(state: &AppState) -> Self {
state.cookie_key.clone()
}
}
pub mod config;