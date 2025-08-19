use std::sync::{Arc, Mutex};

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::io::{EspIOError, Write};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::http::Method;
use log::info;
use wifi::wifi;

static INDEX_HTML: &str = include_str!("index.html");

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_password: &'static str,
}

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    let led_pin = peripherals.pins.gpio1;
    let led = Arc::new(Mutex::new(PinDriver::output(led_pin)?));
    let app_config = CONFIG;

    // 连接Wi-Fi
    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_password,
        peripherals.modem,
        sysloop,
    );

    // 设置 Http 服务器
    let mut server = EspHttpServer::new(&Configuration::default())?;

    server.fn_handler(
        "/",
        Method::Get,
        |request| -> core::result::Result<(), EspIOError> {
            request
                .into_ok_response()?
                .write_all(INDEX_HTML.as_bytes())
                .map(|_| ())
        },
    )?;

    let led_status = led.clone();
    let led_on = led.clone();
    let led_off = led.clone();

    server.fn_handler(
        "/led/status",
        Method::Get,
        move |request| -> core::result::Result<(), EspIOError> {
            let is_high = led_status.lock().unwrap().is_set_high();
            let mut response = request.into_ok_response()?;
            response.write_all(format!("{{\"state\":{}}}", is_high).as_bytes())?;
            Ok(())
        },
    )?;

    server.fn_handler("/led/on", Method::Post, move |request| {
        let _ = led_on.lock().unwrap().set_high();
        request
            .into_ok_response()?
            .write_all("{\"success\":true}".as_bytes())
    })?;

    server.fn_handler("/led/off", Method::Post, move |request| {
        let _ = led_off.lock().unwrap().set_low();
        request
            .into_ok_response()?
            .write_all("{\"success\":true}".as_bytes())
    })?;

    info!("Server awaiting connection");

    loop {
        FreeRtos::delay_ms(1000);
    }
}
