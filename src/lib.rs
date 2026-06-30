mod actor;
pub mod config;
mod net;
mod overlay;
pub mod runtime;

pub mod telemetry {
    use tracing::info;
    use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

    pub fn init() -> anyhow::Result<()> {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_thread_names(false),
            )
            .try_init()?;

        info!("initialized telemetry");

        Ok(())
    }
}
