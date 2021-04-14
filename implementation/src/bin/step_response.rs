#![no_std]
#![no_main]
// Allows for communication back to host during panics and dubugging
// Uncomment when using GDB
// #[allow(unused_imports)]
// use cortex_m_semihosting::{dbg, hprintln};
// use panic_semihosting as _;
use panic_probe as _;

// Allows real time logging back to the host. Unfortunately, this is not compatible  with GDB
use defmt_rtt as _;

use embedded_time::duration::*;
//use implementation::globals::{self, CART_ENCODER, MOTOR_DRIVER, PENDULUM_ENCODER};
use implementation::init;

#[rtic::app(device = stm32f3xx-hal)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        defmt::info!("hello");
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        static mut X: u32 = 0;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        hprintln!("idle").unwrap();

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }
};
