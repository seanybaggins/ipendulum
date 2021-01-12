#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// Allows for communication back to host during panics and dubugging
use panic_semihosting as _;
use cortex_m_semihosting::{dbg, hprintln};

use stm32f3xx_hal as hal;
use hal::prelude::*;

use cortex_m::asm::delay;

use cortex_m_rt::entry;

#[cfg_attr(not(test), entry)]
fn main() -> ! {
    let device_peripherals = hal::pac::Peripherals::take().unwrap();
    // Setting the clock frequency to the maximum supported value
    let mut flash = device_peripherals.FLASH.constrain();
    let mut reset_and_control_clock = device_peripherals.RCC.constrain();
    reset_and_control_clock.cfgr.sysclk(64.mhz()).freeze(&mut flash.acr);

    // Configuring LED
    let mut gpioe = device_peripherals.GPIOE.split(&mut reset_and_control_clock.ahb);
    let mut red_led = gpioe.pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Creating a delay abstraction
    loop {
        red_led.toggle().unwrap();
        delay(8_000_000);
        red_led.toggle().unwrap();
        delay(8_000_000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_two_works() {
        assert_eq!(1, 1)
    }

    #[test]
    fn will_fail() {
        assert_eq!(1, 2);
    }
}
