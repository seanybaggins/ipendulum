#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::u16;

// Allows for communication back to host during panics and dubugging
use cortex_m_semihosting::{dbg, hprintln};
use panic_semihosting as _;

use hal::prelude::*;
use hal::pwm;
use stm32f3xx_hal as hal;

use cortex_m::asm;
use cortex_m_rt::entry;

#[cfg_attr(not(test), entry)]
fn main() -> ! {
    let device_peripherals = hal::pac::Peripherals::take().unwrap();
    // Setting the clock frequency to the maximum supported value
    let mut flash = device_peripherals.FLASH.constrain();
    let mut reset_and_control_clock = device_peripherals.RCC.constrain();
    let clocks = reset_and_control_clock
        .cfgr
        .sysclk(16.mhz())
        .freeze(&mut flash.acr);

    // Configuring LED
    let mut gpioe = device_peripherals
        .GPIOE
        .split(&mut reset_and_control_clock.ahb);
    let mut red_led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Configuring PWM
    let resolution = u16::MAX;
    let frequency = 1.hz();
    let pwm_channel_no_pins = pwm::tim16(device_peripherals.TIM16, resolution, frequency, &clocks);
    let mut gpiob = device_peripherals
        .GPIOB
        .split(&mut reset_and_control_clock.ahb);
    let pb8 = gpiob.pb8.into_af1(&mut gpiob.moder, &mut gpiob.afrh);
    let mut pwm_channel = pwm_channel_no_pins.output_to_pb8(pb8);
    pwm_channel.set_duty(pwm_channel.get_max_duty() / 2);
    //dbg!(pwm_channel.get_max_duty());
    //dbg!(pwm_channel.get_duty());
    pwm_channel.enable();

    // Creating a delay abstraction
    loop {
        red_led.toggle().unwrap();
        asm::delay(8_000_000);
        red_led.toggle().unwrap();
        asm::delay(8_000_000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn will_work() {
        assert_eq!(1, 1)
    }

    #[test]
    fn will_fail() {
        assert_eq!(1, 2);
    }
}
