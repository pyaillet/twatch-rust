#![no_std]
#![no_main]

use esp32_hal::{clock::ClockControl, pac::Peripherals, prelude::*, Delay};
use esp_backtrace as _;
use esp_println::println;

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    loop {
        println!("Hello world!");
        delay.delay_ms(1000u32);
    }
}
