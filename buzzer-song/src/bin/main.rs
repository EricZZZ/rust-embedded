#![no_std]
#![no_main]

use buzzer_song::{
    music::{self, Song},
    pink_panther,
};

use esp_hal::{
    clock::CpuClock,
    ledc::{
        channel::{self, ChannelIFace},
        timer::TimerIFace,
        LSGlobalClkSource, Ledc,
    },
    time::Rate,
};
use esp_hal::{ledc::timer, main};
use esp_hal::{
    ledc::LowSpeed,
    time::{Duration, Instant},
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.3.1
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let buzzer = peripherals.GPIO1;

    let mut ledc = Ledc::new(peripherals.LEDC);

    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty10Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(2000),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, buzzer);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 50,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    loop {
        // Set up a breathing LED: fade from off to on over a second, then
        // from on back off over the next second.  Then loop.
        channel0.start_duty_fade(0, 100, 1000).unwrap();
        while channel0.is_duty_fade_running() {}
        channel0.start_duty_fade(100, 0, 1000).unwrap();
        while channel0.is_duty_fade_running() {}
    }
}

fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
