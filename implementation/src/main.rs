#![no_std]
#![no_main]

// defining panic behavior
use panic_halt as _;

use stm32f3xx_hal::prelude::*;
use cortex_m_semihosting::{hprintln, dbg};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    
    hprintln!("hello world :)");

    loop {
        panic!("Testing panics :)")
    }
}
