use tracing_subscriber::{EnvFilter, fmt};

pub fn init() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=info".into());

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}