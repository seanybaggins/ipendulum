#![no_std]
#![no_main]
mod init;

use core::cell::RefCell;

// Allows for communication back to host during panics and dubugging
// Uncomment when using GDB
#[allow(unused_imports)]
use cortex_m_semihosting::{dbg, hprintln};
use panic_semihosting as _;

// Allows real time logging back to the host. Unfortunately, this is not compatible  with the device itself
use hal::{
    gpio::ExtiPin,
    interrupt,
    pac::{Interrupt, NVIC},
};
use stm32f3xx_hal as hal;

use init::{CartEncoder, PendulumEncoder};

use cortex_m::asm;
use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::global_logger;

static CART_ENCODER: Mutex<RefCell<Option<CartEncoder>>> = Mutex::new(RefCell::new(None));
static PENDULUM_ENCODER: Mutex<RefCell<Option<PendulumEncoder>>> = Mutex::new(RefCell::new(None));

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
    //  eh   .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Enabling interrupts
    unsafe {
        NVIC::unmask(Interrupt::EXTI1);
        //NVIC::unmask(Interrupt::EXTI15_10);
    }
    // Blinking the LED for debug purposes
    loop {
        asm::wfi();
    }
}

// Cart Encoder Interrupt
#[interrupt]
fn EXTI1() {
    //hprintln!("Cart Interrupt triggered").unwrap();
    interrupt_free(|cs| {
        let mut cart_encoder = CART_ENCODER.borrow(cs).borrow_mut();
        let cart_encoder = cart_encoder
            .as_mut()
            .expect("EXTI1 interrupt was called and CartEncoder was not intialized");

        // Update the angle and direction state of the encoder
        cart_encoder
            .update()
            .expect("Failed to update the position of the encoder");

        defmt::info!("angle: {:f32}", cart_encoder.angle().radians());
        //hprintln!("angle: {}", cart_encoder.angle().radians()).unwrap();

        cart_encoder
            .hardware()
            .pin_a()
            .clear_interrupt_pending_bit()
    });
}

// Pendulum Encoder Interrupt
#[interrupt]
fn EXTI15_10() {
    defmt::info!("")
}
