use spectral::config::load_config;

fn main() {
    let config = load_config("config.toml").unwrap();
    println!("{:?}", config);
}
