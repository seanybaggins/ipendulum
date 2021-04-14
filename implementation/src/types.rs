use hal::gpio::{
    gpioa::{PA0, PA1, PA2, PA3},
    gpiod::{PD1, PD3, PD4},
    Floating, Input, Output, PushPull, AF4,
};
use hal::pwm::{PwmChannel, WithPins, TIM8_CH4};
use stm32f3xx_hal as hal;

pub type CartEncoder = es38::Encoder<CartEncIn1, CartEncIn2>;
type CartEncIn1 = PA1<Input<Floating>>;
type CartEncIn2 = PA3<Input<Floating>>;

pub type PendulumEncoder = es38::Encoder<PendulumEncIn1, PendulumEncIn2>;
type PendulumEncIn1 = PA0<Input<Floating>>;
type PendulumEncIn2 = PA2<Input<Floating>>;

pub type MotorDriver = l298n::Motor<MotorDriverOut1, MotorDriverOut2, MotorDriverPwm>;
type MotorDriverOut1 = PD3<Output<PushPull>>;
type MotorDriverOut2 = PD4<Output<PushPull>>;
type MotorDriverPwm = PwmChannel<TIM8_CH4, WithPins>;
pub type MotorDriverPwmPin = PD1<AF4>;
