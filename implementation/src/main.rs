#![no_std]
#![no_main]
mod init;

use core::cell::RefCell;
use core::convert::TryInto;
use es38::Velocity;
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

macro_rules! get_global_ref {
    ($global_item:ident, $cs:ident) => {
        $global_item
            .borrow($cs)
            .borrow()
            .as_ref()
            .expect("failed to get global reference")
    };
}

macro_rules! get_global_ref_mut {
    ($global_item:ident, $cs:ident) => {
        $global_item
            .borrow($cs)
            .borrow_mut()
            .as_mut()
            .expect("failed to get global mutible reference")
    };
}

macro_rules! update_encoder {
    ($encoder:ident, $cs:ident) => {{
        defmt::trace!("encoder update");

        let millisec_since_epoch = get_milli_sec_since_epoch($cs);

        // Update the angle and direction state of the encoder
        get_global_ref_mut!($encoder, $cs)
            .update(millisec_since_epoch)
            .unwrap();
        get_global_ref_mut!($encoder, $cs)
            .hardware()
            .pin_a()
            .clear_interrupt_pending_bit();
        get_global_ref_mut!($encoder, $cs)
            .hardware()
            .pin_b()
            .clear_interrupt_pending_bit();
    }};
}

fn get_milli_sec_since_epoch(cs: &CriticalSection) -> Milliseconds<u32> {
    defmt::trace!("get_milli_sec_since_epoch");
    get_global_ref!(STOPWATCH, cs)
        .try_now()
        .unwrap()
        .duration_since_epoch()
        .try_into()
        .unwrap()
}

#[entry]
fn main() -> ! {
    defmt::trace!("main");
    let init::Hardware {
        pendulum_encoder,
        cart_encoder,
        motor_driver: _,
        stopwatch,
    } = init::Hardware::take();

    // handing the hardware over to a global context do they can be accessed within an interrupt
    interrupt_free(|cs| {
        CART_ENCODER.borrow(cs).replace(Some(cart_encoder));
        PENDULUM_ENCODER.borrow(cs).replace(Some(pendulum_encoder));
        STOPWATCH.borrow(cs).replace(Some(stopwatch));
    });

    defmt::debug!("Unmasking interrupts");
    unsafe {
        NVIC::unmask(Interrupt::EXTI1);
        NVIC::unmask(Interrupt::EXTI3);
        //NVIC::unmask(Interrupt::EXTI15_10);
    }

    defmt::trace!("looping forever");
    loop {
        let time_since_epoch_milli_sec = interrupt_free(|cs| get_milli_sec_since_epoch(cs));

        let (cart_angle, cart_velocity) = interrupt_free(|cs| {
            let cart_angle = get_global_ref!(CART_ENCODER, cs).angle().clone();
            let cart_velocity = get_global_ref_mut!(CART_ENCODER, cs)
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
