#![no_std]
#![no_main]

/// All things related to the intialization of the micro controller
mod init;
mod utils;

use core::cell::RefCell;
use core::convert::TryInto;
use es38::{Angle, Velocity};
// Allows for communication back to host during panics and dubugging
// Uncomment when using GDB
// #[allow(unused_imports)]
// use cortex_m_semihosting::{dbg, hprintln};
// use panic_semihosting as _;
use panic_probe as _;

// Allows real time logging back to the host. Unfortunately, this is not compatible  with GDB
use defmt_rtt as _;

use hal::{
    gpio::ExtiPin,
    interrupt,
    pac::{Interrupt, NVIC},
    time::{duration::*, Clock},
};
use stm32f3xx_hal as hal;

use init::{CartEncoder, PendulumEncoder, StopWatch};

use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::{CriticalSection, Mutex};
use cortex_m_rt::entry;

static CART_ENCODER: Mutex<RefCell<Option<CartEncoder>>> = Mutex::new(RefCell::new(None));
static PENDULUM_ENCODER: Mutex<RefCell<Option<PendulumEncoder>>> = Mutex::new(RefCell::new(None));
static STOPWATCH: Mutex<RefCell<Option<StopWatch>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    defmt::trace!("main");
    setup();
    defmt::trace!("looping forever");
    loop {
        let time_since_epoch_milli_sec = interrupt_free(|cs| utils::get_milli_sec_since_epoch(cs));

        let (cart_angle, cart_velocity) = interrupt_free(|cs| {
            let cart_angle = utils::get_global_ref!(CART_ENCODER, cs).angle().clone();
            let cart_velocity = utils::get_global_ref_mut!(CART_ENCODER, cs)
                .velocity(cart_angle, time_since_epoch_milli_sec);

            (cart_angle, cart_velocity)
        });

        let cart_degrees_per_sec = match cart_velocity.degrees_per_sec() {
            Ok(cart_degrees_per_sec) => cart_degrees_per_sec,
            // Create a local one off velocity struct until the intial time and final time have
            // been correctly updated
            Err(es38::Error::VelocityArithmeticOverflowWouldOccur) => {
                defmt::trace!("Calculating 1 off velocity");
                let delta_time_milli_sec = Milliseconds(
                    StopWatch::max_time_milli_sec()
                        - *cart_velocity.initial_time_since_epoch_milli_sec().integer()
                        + *cart_velocity.final_time_since_epoch_milli_sec().integer(),
                );
                Velocity::new(
                    Milliseconds(0),
                    delta_time_milli_sec,
                    cart_velocity.initial_angle(),
                    cart_velocity.final_angle(),
                )
                .degrees_per_sec()
                .expect("failure in re-evaluation of velocity")
            }
        };

        defmt::debug!("cart_velocity: {}", cart_velocity);
        defmt::debug!("cart_degrees_per_sec: {}", cart_degrees_per_sec);
        // defmt::debug!("cart_angle = {} deg", cart_angle.degrees());

        cortex_m::asm::delay(16_000_000)
    }
}

// Cart Encoder Interrupt
#[interrupt]
fn EXTI1() {
    defmt::trace!("Interrupt EXTI1");
    interrupt_free(|cs| update_encoder!(CART_ENCODER, cs))
}

#[interrupt]
fn EXTI3() {
    defmt::trace!("Interrupt EXTI3");
    interrupt_free(|cs| update_encoder!(CART_ENCODER, cs))
}

// Pendulum Encoder Interrupt
#[interrupt]
fn EXTI15_10() {
    defmt::trace!("Interrupt EXTI15_10");
}
