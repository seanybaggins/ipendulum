#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod leds;

// Allows for communication back to host during panics and dubugging
use panic_semihosting as _;
use cortex_m_semihosting::{dbg, hprintln};

use stm32f3xx_hal as hal;
use hal::{prelude::*, serial::Serial};

use cortex_m_rt::entry;

#[cfg_attr(not(test), entry)]
fn main() -> ! {
    let device_peripherals = hal::pac::Peripherals::take().unwrap();
    let mut reset_and_control_clock = device_peripherals.RCC.constrain();
    let mut gpiob = device_peripherals.GPIOB.split(&mut reset_and_control_clock.ahb);
    
    loop {}
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
