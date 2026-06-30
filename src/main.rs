use spectral::{config::load_config, telemetry};
use tracing::info;

fn main() -> anyhow::Result<()> {
    telemetry::init()?;
    let config = load_config("config.toml").unwrap();
    info!("{:?}", config);

    Ok(())
}
