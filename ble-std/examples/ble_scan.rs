use anyhow::Result;
use esp32_nimble::{BLEDevice, BLEScan};
use esp_idf_svc::hal::task::block_on;
use log::info;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Debug);

    block_on(async {
        let ble_device = BLEDevice::take();
        let mut ble_scan = BLEScan::new();
        ble_scan.active_scan(true).interval(100).window(99);

        ble_scan
            .start(ble_device, 5000, |device, data| {
                info!("Advertised Device: ({:?}, {:?})", device, data);
                None::<()>
            })
            .await?;
        info!("Scan finished");

        Ok(())
    })
}
