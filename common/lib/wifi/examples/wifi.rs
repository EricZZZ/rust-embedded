use anyhow::{bail, Result};
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::prelude::Peripherals};
use wifi::wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_password: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let app_config = CONFIG;
    let _wifi = match wifi(
        app_config.wifi_ssid,
        app_config.wifi_password,
        peripherals.modem,
        sysloop,
    ) {
        Ok(wifi) => {
            println!("Connected to Wi-Fi network!");
            wifi
        }
        Err(e) => {
            bail!("Failed to initialize wifi: {:?}", e);
        }
    };
    Ok(())
}
