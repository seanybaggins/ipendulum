use crate::globals::CART_ENCODER;
use crate::update_encoder;
use cortex_m::interrupt::free as interrupt_free;
use hal::interrupt;
use stm32f3xx_hal as hal;

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
