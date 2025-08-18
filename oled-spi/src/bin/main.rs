#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::fmt::Write;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::{Baseline, Text},
};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::gpio::Output;
use esp_hal::gpio::OutputConfig;
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::master::Spi;
use esp_hal::spi::Mode as SpiMode;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_println::print;
use ssd1306::{prelude::*, Ssd1306};
use tinybmp::Bmp;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    print!("Embassy initialized!");

    // Initialize SPI
    let spi_bus = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(2))
            .with_mode(SpiMode::_0),
    )
    .unwrap()
    //CLK
    .with_sck(peripherals.GPIO2)
    //DIN
    .with_mosi(peripherals.GPIO3);
    let cs = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    // 关键：定义RST引脚
    let mut rst = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());

    // 创建 SSD1306 驱动对象
    // 使用 spi_bus 和 cs 引脚
    let spi_dev = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap();
    let interface = SPIInterface::new(spi_dev, dc);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // 手动复位一下，确保在初始化前屏幕被正确复位
    // rst.set_low();
    // Timer::after(Duration::from_millis(10)).await;
    // rst.set_high();
    // Timer::after(Duration::from_millis(10)).await;

    display.init().unwrap();
    display.flush().unwrap();
    // display.clear().unwrap();

    // 显示文字
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("功德+1", Point::new(108, 0), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    // 加载并显示图片
    // let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    // let im = Image::new(&raw, Point::new(32, 0));
    // im.draw(&mut display).unwrap();

    // 使用 tinybmp 库加载 BMP 图片
    let bmp_data = include_bytes!("./output.bmp");
    let bmp: Bmp<BinaryColor> = Bmp::from_slice(bmp_data).unwrap();

    let bmp_data_1 = include_bytes!("./output1.bmp");
    let bmp_1: Bmp<BinaryColor> = Bmp::from_slice(bmp_data_1).unwrap();

    let bmp_data2 = include_bytes!("./output2.bmp");
    let bmp2: Bmp<BinaryColor> = Bmp::from_slice(bmp_data2).unwrap();

    let bmps = [bmp, bmp_1, bmp2, bmp_1, bmp];
    let mut idx = 0;
    let mut y = 0;
    loop {
        idx = (idx + 1) % bmps.len();
        if idx == 0 {
            y = 0;
        } else if idx == 1 {
            y = 4;
        } else if idx == 2 {
            y = 8;
        } else if idx == 3 {
            y = 4;
        } else {
            y = 0;
        }
        let image = Image::new(&bmps[idx], Point::new(32, y));
        image.draw(&mut display).unwrap();
        display.flush().unwrap();
        Timer::after(Duration::from_millis(200)).await;
    }

    // Spam some characters to the display
    // for c in 97..123 {
    //     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    // }
    // for c in 65..91 {
    //     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    // }

    // // The `write!()` macro is also supported
    // write!(display, "Hello, {}", "world");
    // let mut x_pos = 0;
    // let mut direction = 1;

    // let mut millis = 0;
    // let mut second = 0;
    // let mut minute = 0;
    // let mut hour = 0;
    // loop {
    //     display.clear_buffer();
    //     let mut time_str = heapless::String::<64>::new();
    //     write!(&mut time_str, "{:02}:{:02}:{:02}", hour, minute, second).unwrap();
    //     // 绘制一个移动的圆
    //     let circle_style = PrimitiveStyleBuilder::new()
    //         .stroke_color(BinaryColor::Off)
    //         .stroke_width(1)
    //         .fill_color(BinaryColor::On)
    //         .build();

    //     let circle = Circle::new(Point { x: x_pos, y: 16 }, 10).into_styled(circle_style);

    //     circle.draw(&mut display).unwrap();

    //     // 更新位置和方向
    //     x_pos += direction;
    //     if x_pos >= 128 - 10 {
    //         direction = -1;
    //     } else if x_pos <= 0 {
    //         direction = 1;
    //     }

    //     // 显示文字
    //     let text_style = MonoTextStyleBuilder::new()
    //         .font(&FONT_6X10)
    //         .text_color(BinaryColor::On)
    //         .build();

    //     Text::with_baseline(&time_str, Point::new(0, 50), text_style, Baseline::Top)
    //         .draw(&mut display)
    //         .unwrap();
    //     // 更新时间
    //     millis += 50;
    //     if millis >= 1000 {
    //         millis = 0;
    //         second += 1;
    //     }
    //     if second >= 60 {
    //         second = 0;
    //         minute += 1;
    //     }
    //     if minute >= 60 {
    //         minute = 0;
    //         hour += 1;
    //     }
    //     if hour >= 24 {
    //         hour = 0;
    //     }

    //     // 刷新显示
    //     display.flush().unwrap();
    //     // 等待一段时间
    //     Timer::after(Duration::from_millis(50)).await;
    // }
}
