//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO15)

#![no_std]
#![no_main]

use core::fmt::Write;

use esp32_hal::{
    clock::ClockControl,
    gpio::{Gpio2, IO},
    gpio_types::{Output, PushPull},
    pac::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay, Rtc
};
use esp_backtrace as _;
use xtensa_lx_rt::entry;

struct Blinker {
    led: Gpio2<Output<PushPull>>,
    delay: Delay,
}

impl Blinker {
    fn run(&mut self) {
        loop {
            self.led.toggle().unwrap();
            self.delay.delay_ms(500u32);
        }
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt = timer_group0.wdt;
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);

    let delay = Delay::new(&clocks);

    // Disable MWDT and RWDT (Watchdog) flash boot protection
    wdt.disable();
    rtc.rwdt.disable();

    // Set GPIO15 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = io.pins.gpio2.into_push_pull_output();

    let mut blinker = Blinker { led, delay };
    blinker.run();

    let gpio2 = led.into_floating_input();

    loop {}
}
