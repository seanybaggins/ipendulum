#![no_std]
#![no_main]

// defining panic behavior
use panic_semihosting as _;

use stm32f3xx_hal::prelude::*;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    
    loop {
        panic!("Testing panics :)")
    }
}
