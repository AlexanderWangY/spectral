use std::{fs, path::PathBuf};

use spectral::config::load_config;

#[test]
fn loads_config_from_path_and_exposes_fields() {
    let path = temp_path();

    fs::write(
        &path,
        r#"
[cluster]
min_p = 3
inflate_high_watermark = 0.9
deflate_low_watermark = 0.4
hard_max_load = 100
resize_cooldown_ms = 5000
t_stable_ms = 10000
t_emergency_ms = 1000

[membership]
protocol_period_ms = 1000
ping_timeout_ms = 200
indirect_probes = 3
suspect_timeout_ms = 5000
lifeguard_max_lhm = 8
stable_periods = 4

[catalog]
default_ttl_ms = 60000
host_alive_grace_ms = 10000
tombstone_retention_ms = 300000

[gossip]
interval_ms = 1000
fanout = 3
digest_mode = true
"#,
    )
    .unwrap();

    let config = load_config(&path).unwrap();

    assert_eq!(config.cluster.min_p, 3);
    assert_eq!(config.cluster.deflate_low_watermark, 0.4);
    assert!(config.gossip.digest_mode);

    let _ = fs::remove_file(path);
}

fn temp_path() -> PathBuf {
    std::env::temp_dir().join(format!("spectral-config-{}.toml", std::process::id()))
}
