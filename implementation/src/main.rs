#![no_std]
#![no_main]
mod init;

use core::cell::RefCell;

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
};
use stm32f3xx_hal as hal;

use init::{CartEncoder, PendulumEncoder};

use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;

static CART_ENCODER: Mutex<RefCell<Option<CartEncoder>>> = Mutex::new(RefCell::new(None));
static PENDULUM_ENCODER: Mutex<RefCell<Option<PendulumEncoder>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    defmt::trace!("main");
    let init::Hardware {
        pendulum_encoder,
        cart_encoder,
        motor_driver: _,
    } = init::Hardware::take();

    // handing the hardware over to a global context do they can be accessed within an interrupt
    interrupt_free(|cs| {
        CART_ENCODER.borrow(cs).replace(Some(cart_encoder));
        PENDULUM_ENCODER.borrow(cs).replace(Some(pendulum_encoder));
    });

    defmt::debug!("Unmasking interrupts");
    unsafe {
        NVIC::unmask(Interrupt::EXTI1);
        NVIC::unmask(Interrupt::EXTI3);
        //NVIC::unmask(Interrupt::EXTI15_10);
    }

    defmt::trace!("looping forever");
    loop {}
}

fn update_cart(cs: &cortex_m::interrupt::CriticalSection) {
    defmt::trace!("update cart");
    let mut cart_encoder = CART_ENCODER.borrow(cs).borrow_mut();
    let cart_encoder = cart_encoder
        .as_mut()
        .expect("EXTI1 interrupt was called and CartEncoder was not intialized");

    // Update the angle and direction state of the encoder
    cart_encoder
        .update()
        .expect("Failed to update the position of the encoder");
    defmt::debug!("counts: {}", cart_encoder.angle().counts());
    defmt::debug!("theta: {}", cart_encoder.angle().degrees());

    cart_encoder
        .hardware()
        .pin_a()
        .clear_interrupt_pending_bit();

    cart_encoder
        .hardware()
        .pin_b()
        .clear_interrupt_pending_bit();
}
// Cart Encoder Interrupt
#[interrupt]
fn EXTI1() {
    defmt::trace!("Interrupt EXTI1");
    interrupt_free(|cs| update_cart(cs))
}

#[interrupt]
fn EXTI3() {
    defmt::trace!("Interrupt EXTI3");
    interrupt_free(|cs| update_cart(cs))
}
// Pendulum Encoder Interrupt
#[interrupt]
fn EXTI15_10() {}
