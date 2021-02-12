#![no_std]
#![no_main]
mod init;

use core::{borrow::Borrow, cell::RefCell, future::pending};
use core::u16;

// Allows for communication back to host during panics and dubugging
use cortex_m_semihosting::{dbg, hprintln};
use panic_semihosting as _;

use stm32f3xx_hal as hal;
use hal::{
    interrupt,
    pac::Interrupt,
};

use init::{CartEncoder, PendulumEncoder};


//use cortex_m::asm;
use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

static CART_ENCODER: Mutex<RefCell<Option<CartEncoder>>> =
    Mutex::new(RefCell::new(None));
static PENDULUM_ENCODER: Mutex<RefCell<Option<PendulumEncoder>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    
    let init::Hardware {
        pendulum_encoder,
        cart_encoder,
        motor_driver,
    } = init::Hardware::get();

    // handing the hardware over to a global context do they can be accessed within an interrupt
    interrupt_free(|cs| {
        CART_ENCODER.borrow(cs).replace(Some(cart_encoder));
        PENDULUM_ENCODER.borrow(cs).replace(Some(pendulum_encoder));
    });

    // Configuring a LED
    // let mut red_led = gpioe
    //     .pe9
    //     .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Blinking the LED for debug purposes
    loop {
        // red_led.toggle().unwrap();
        // asm::delay(8_000_000);
        // red_led.toggle().unwrap();
        // asm::delay(8_000_000);
    }
}

#[interrupt]
fn EXTI0() {
    hprintln!("EXTI0 triggered").unwrap();
}

#[interrupt]
fn EXTI1() {
    hprintln!("EXTI1 triggered").unwrap();
}
