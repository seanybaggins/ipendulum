#![no_std]
#![no_main]
mod globals;
mod init;
mod interrupts;
mod types;
mod utils;

use es38::Velocity;
// Allows for communication back to host during panics and dubugging
// Uncomment when using GDB
// #[allow(unused_imports)]
// use cortex_m_semihosting::{dbg, hprintln};
// use panic_semihosting as _;
use panic_probe as _;

// Allows real time logging back to the host. Unfortunately, this is not compatible  with GDB
use defmt_rtt as _;

use cortex_m::interrupt::free as interrupt_free;
use cortex_m_rt::entry;
use embedded_time::duration::*;
use globals::{CART_ENCODER, MOTOR_DRIVER};

#[entry]
fn main() -> ! {
    init::setup();
    defmt::trace!("looping forever");
    loop {
        let time_since_epoch_milli_sec = interrupt_free(|cs| utils::get_milli_sec_since_epoch(cs));

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
                    globals::StopWatch::max_time_milli_sec()
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
