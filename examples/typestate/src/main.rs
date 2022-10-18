#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, pac::Peripherals, prelude::*, IO, Delay};
use esp_backtrace as _;
use xtensa_lx_rt::entry;
use esp_println::println;


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio4.into_push_pull_output();

    loop {
        led.toggle().unwrap();
        delay.delay_ms(1000u32);
        println!("Led is high: {}", led.is_high());

    }
}

