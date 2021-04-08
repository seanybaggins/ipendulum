use hal::gpio::{
    gpioa::{PA1, PA3},
    gpiob::{PB6, PB7, PB8},
    gpioe::{PE13, PE15},
    Input, Output, PullUp, PushPull, AF1,
};
use hal::pwm::{PwmChannel, WithPins, TIM16_CH1};
use stm32f3xx_hal as hal;
pub type PendulumEncoder = es38::Encoder<PendulumEncIn1, PendulumEncIn2>;
type PendulumEncIn1 = PE13<Input<PullUp>>;
type PendulumEncIn2 = PE15<Input<PullUp>>;

pub type CartEncoder = es38::Encoder<CartEncIn1, CartEncIn2>;
type CartEncIn1 = PA1<Input<PullUp>>;
type CartEncIn2 = PA3<Input<PullUp>>;

pub type MotorDriver = l298n::Motor<MotorDriverOut1, MotorDriverOut2, MotorDriverPwm>;
type MotorDriverOut1 = PB6<Output<PushPull>>;
type MotorDriverOut2 = PB7<Output<PushPull>>;
type MotorDriverPwm = PwmChannel<TIM16_CH1, WithPins>;
pub type MotorDriverPwmPin = PB8<AF1>;
