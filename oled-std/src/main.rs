use anyhow::Result;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::{Baseline, Text};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::config::MODE_0;
use esp_idf_hal::spi::{config, SpiConfig, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::units::MegaHertz;

use log::info;

use ssd1306::prelude::DisplayRotation;
use ssd1306::prelude::SPIInterface;
use ssd1306::size::DisplaySize128x64;
use ssd1306::Ssd1306;

use embedded_graphics::Drawable;
use ssd1306::prelude::*;

// 添加u8g2-fonts相关导入
use u8g2_fonts::{
    fonts,
    FontRenderer,
};
use u8g2_fonts::types::{
    FontColor,
    VerticalPosition,
};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    // 初始化 SPI 引脚和控制引脚
    let sck = peripherals.pins.gpio2;
    let mosi = peripherals.pins.gpio3;
    let cs = peripherals.pins.gpio7;
    let dc = PinDriver::output(peripherals.pins.gpio8)?;
    let mut rst = PinDriver::output(peripherals.pins.gpio4)?;

    let spi_config = SpiConfig::new()
        .baudrate(MegaHertz(2).into())
        .data_mode(MODE_0);
    let driver_config = config::DriverConfig::new();

    let spi_driver = SpiDriver::new(peripherals.spi2, sck, mosi, None::<esp_idf_hal::gpio::AnyIOPin>, &driver_config)?;

    // 创建 SPI 设备驱动
    let spi = SpiDeviceDriver::new(&spi_driver, Some(cs), &spi_config)?;

    // 创建 SSD1306 SPI 接口
    let interface = SPIInterface::new(spi, dc);

    // 硬件复位屏幕
    rst.set_low().unwrap();
    FreeRtos::delay_ms(10);
    rst.set_high().unwrap();
    FreeRtos::delay_ms(10);

    // 创建 SSD1306 设备
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // 初始化屏幕
    display.init().unwrap();
    FreeRtos::delay_ms(100);

    info!("开始初始化显示内容...");
    
    // 刷新前，你需要先清空屏幕
    display.clear_buffer();
    
    // 使用ASCII字体显示英文
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello ESP32!", Point::new(10, 5), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
        
    Text::with_baseline("OLED Test", Point::new(10, 20), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    
    // 使用u8g2中文字体显示中文
    let font = FontRenderer::new::<fonts::u8g2_font_wqy12_t_gb2312>();
    
    font.render(
        "中文",
        Point::new(10, 35),
        VerticalPosition::Top,
        FontColor::Transparent(BinaryColor::On),
        &mut display,
    ).unwrap();
    
    font.render(
        "功德+1",
        Point::new(10, 52),
        VerticalPosition::Top,
        FontColor::Transparent(BinaryColor::On),
        &mut display,
    ).unwrap();
    
    info!("内容绘制完成，开始刷新屏幕...");
    
    // 刷新屏幕
    display
        .flush()
        .map_err(|e| anyhow::anyhow!("Failed to flush display: {:?}", e))?;

    info!("SPI 屏幕已点亮！显示内容已刷新。");

    loop {
        FreeRtos::delay_ms(1000);
    }
}
