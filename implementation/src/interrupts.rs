use crate::encoder;
use crate::globals::{CART_ENCODER, PENDULUM_ENCODER};
use hal::interrupt;
use stm32f3xx_hal as hal;

#[interrupt]
fn EXTI0() {
    defmt::trace!("Interrupt EXTI0");
    encoder::update(&mut PENDULUM_ENCODER.unwrap());
}

#[interrupt]
fn EXTI1() {
    defmt::trace!("Interrupt EXTI1");
    encoder::update(&mut CART_ENCODER.unwrap());
}

#[interrupt]
fn EXTI2_TSC() {
    defmt::trace!("Interrupt EXTI2");
    encoder::update(&mut PENDULUM_ENCODER.unwrap());
}

#[interrupt]
fn EXTI3() {
    defmt::trace!("Interrupt EXTI3");
    encoder::update(&mut CART_ENCODER.unwrap());
}
