#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// defining panic behavior
use panic_semihosting as _;

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