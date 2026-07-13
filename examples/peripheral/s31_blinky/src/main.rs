//! ESP32-S31 GPIO blinky example
//!
//! This is a minimal blinky program that toggles an LED connected to GPIO1.
//! It serves as the first firmware test for ESP32-S31 support in esp-hal.
//!
//! Once the esp32s31 PAC crate is available in esp-rs/esp-pacs, this can be
//! built with:
//! ```bash
//! cargo build --release --features esp32s31 \
//!     --target riscv32imac-unknown-none-elf
//! ```
//! And flashed with:
//! ```bash
//! cargo espflash flash --monitor --chip ESP32-S31
//! ```

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output},
    main,
    time::Delay,
};
use panic_halt as _;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Configure GPIO1 as an output (onboard LED on Korvo-1 board)
    let mut led = Output::new(peripherals.GPIO1, Level::High);

    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(500);
    }
}
