use std::{error::Error, fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub cluster: Cluster,
    pub membership: Membership,
    pub catalog: Catalog,
    pub gossip: Gossip,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cluster {
    pub min_p: u64,
    pub inflate_high_watermark: f64,
    pub deflate_low_watermark: f64,
    pub hard_max_load: u64,
    pub resize_cooldown_ms: u64,
    pub t_stable_ms: u64,
    pub t_emergency_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Membership {
    pub protocol_period_ms: u64,
    pub ping_timeout_ms: u64,
    pub indirect_probes: u64,
    pub suspect_timeout_ms: u64,
    pub lifeguard_max_lhm: u64,
    pub stable_periods: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub default_ttl_ms: u64,
    pub host_alive_grace_ms: u64,
    pub tombstone_retention_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gossip {
    pub interval_ms: u64,
    pub fanout: u64,
    pub digest_mode: bool,
}

pub fn load_config(path: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}
