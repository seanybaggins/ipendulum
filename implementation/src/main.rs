#![no_std]
#![no_main]

mod panic_behavior;

// For setting up our debugger
use rtt_target::{rprintln, rtt_init_print};

use stm32f3xx_hal::prelude::*;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Set up for debugger
    rtt_init_print!();

    loop {
        panic!("Testing panics :)")
    }
}
