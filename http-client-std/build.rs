#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_password: &'static str,
}

fn main() {
    // Check if the `cfg.toml` file exists and has been filled out.
    if !std::path::Path::new("cfg.toml").exists() {
        println!("cfg.toml file not found. Please create it and fill out the required fields.");
    }

    embuild::espidf::sysenv::output();
}
