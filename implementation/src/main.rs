#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
mod encoders;

use core::cell::RefCell;
use core::u16;

// Allows for communication back to host during panics and dubugging
use cortex_m_semihosting::{dbg, hprintln};
use panic_semihosting as _;

use hal::gpio::gpioa::{PA1, PA3};
use hal::gpio::gpioe::{PE13, PE15};
use hal::gpio::{Input, PullUp};
use hal::interrupt;
use hal::interrupt::{EXTI0, EXTI1};
use hal::prelude::*;
use hal::pwm;
use stm32f3xx_hal as hal;

use cortex_m::asm;
use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

use encoders::Encoder;

static CART_ENCODER: Mutex<RefCell<Option<Encoder<PA3<Input<PullUp>>, PA1<Input<PullUp>>>>>> =
    Mutex::new(RefCell::new(None));
static PENDULUM_ENCODER: Mutex<RefCell<Option<Encoder<PE15<Input<PullUp>>, PE13<Input<PullUp>>>>>> =
    Mutex::new(RefCell::new(None));

#[cfg_attr(not(test), entry)]
fn main() -> ! {
    // Typicall aquiring of board singleton and setting the clock speed
    let device_peripherals = hal::pac::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let mut reset_and_control_clock = device_peripherals.RCC.constrain();
    let clocks = reset_and_control_clock
        .cfgr
        .sysclk(16.mhz())
        .freeze(&mut flash.acr);

    // Pin ports in use
    let mut gpiob = device_peripherals
        .GPIOB
        .split(&mut reset_and_control_clock.ahb);
    let mut gpioe = device_peripherals
        .GPIOE
        .split(&mut reset_and_control_clock.ahb);
    let mut gpioa = device_peripherals
        .GPIOA
        .split(&mut reset_and_control_clock.ahb);

    // Configuring motor
    let pwm_channel_no_pins = pwm::tim16(
        device_peripherals.TIM16,
        u16::MAX, // resolution
        50.hz(),  // frequency
        &clocks,
    );
    let pb8 = gpiob.pb8.into_af1(&mut gpiob.moder, &mut gpiob.afrh);
    let pwm_ena_pb8 = pwm_channel_no_pins.output_to_pb8(pb8);
    let in_1_pb6 = gpiob
        .pb6
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let in_2_pb7 = gpiob
        .pb7
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut motor = l298n::Motor::new(in_1_pb6, in_2_pb7, pwm_ena_pb8);
    motor.set_duty(motor.get_max_duty() / 4);
    motor.forward();

    // Setting the global encoder for the cart
    let a_pa3 = gpioa
        .pa3
        .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
    let b_pa1 = gpioa
        .pa1
        .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
    interrupt_free(|cs| {
        CART_ENCODER
            .borrow(cs)
            .replace_with(|_| Some(Encoder::new(a_pa3, b_pa1)))
    });

    // Setting the global encoder for the pendulum
    let a_pe15 = gpioe
        .pe15
        .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
    let b_pe13 = gpioe
        .pe13
        .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
    interrupt_free(|cs| {
        PENDULUM_ENCODER
            .borrow(cs)
            .replace_with(|_| Some(Encoder::new(a_pe15, b_pe13)))
    });

    // Configuring a LED
    let mut red_led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Blinking the LED for debug purposes
    loop {
        red_led.toggle().unwrap();
        asm::delay(8_000_000);
        red_led.toggle().unwrap();
        asm::delay(8_000_000);
    }
}

#[interrupt]
fn EXTI0() {
    interrupt_free(|cs| {
        
    })
}

#[interrupt]
fn EXTI1() {
    
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
