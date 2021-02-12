use stm32f3xx_hal as hal;
use hal::prelude::*;
use hal::{
    gpio::{
        gpioa::{self, PA1, PA3},
        gpiob::{self, PB6, PB7, PB8},
        gpioe::{self, PE13, PE15},
    },
    gpio::{AF1, Input, PullUp, PushPull, Output},
    pwm,
    pwm::{PwmChannel, WithPins, TIM16_CH1},
    pac,
    pac::TIM16,
    rcc::Clocks,
};

pub type PendulumEncoder = es38::Encoder<PendulumEncoderIn1, PendulumEncoderIn2>;
type PendulumEncoderIn1 = PE13<Input<PullUp>>;
type PendulumEncoderIn2 = PE15<Input<PullUp>>;

pub type CartEncoder = es38::Encoder<CartEncoderIn1, CartEncoderIn2>;
type CartEncoderIn1 = PA1<Input<PullUp>>;
type CartEncoderIn2 = PA3<Input<PullUp>>;

pub type MotorDriver = l298n::Motor<MotorDriverOut1, MotorDriverOut2, MotorDriverPwm>;
type MotorDriverOut1 = PB6<Output<PushPull>>;
type MotorDriverOut2 = PB7<Output<PushPull>>;
type MotorDriverPwm = PwmChannel<TIM16_CH1, WithPins>;
type MotorDriverPwmPin = PB8<AF1>;

pub struct Hardware {
    pub pendulum_encoder: PendulumEncoder,
    pub cart_encoder: CartEncoder,
    pub motor_driver: MotorDriver,
}

impl Hardware {
    pub fn get() -> Self {
        // Typical acquiring of board singleton and setting the clock speed
        let device_peripherals = pac::Peripherals::take().unwrap();
        let mut flash = device_peripherals.FLASH.constrain();
        let mut reset_and_control_clock = device_peripherals.RCC.constrain();
        let clocks = reset_and_control_clock
            .cfgr
            .sysclk(16.mhz())
            .freeze(&mut flash.acr);

        // Pin ports in use
        let mut gpiob = device_peripherals
            .GPIOB
            .split(&mut reset_and_control_clock.ahb);
        let mut gpioe = device_peripherals
            .GPIOE
            .split(&mut reset_and_control_clock.ahb);
        let mut gpioa = device_peripherals
            .GPIOA
            .split(&mut reset_and_control_clock.ahb);

        let (motor_driver, _) = Self::setup_motor(device_peripherals.TIM16, &clocks, gpiob);
        let pendulum_encoder = Self::setup_pendulum_encoder(gpioe);
        let cart_encoder = Self::setup_cart_encoder(gpioa);
        
    }

    /// Configures the motor
    fn setup_motor(timer: TIM16, clocks: &Clocks, mut gpiob: gpiob::Parts) -> MotorDriver {
        let pwm_channel_no_pins = pwm::tim16(
            timer,
            u16::MAX, // resolution
            50.hz(),  // frequency
            &clocks,
        );

        let pb8: MotorDriverPwmPin = gpiob.pb8.into_af1(&mut gpiob.moder, &mut gpiob.afrh);
        let pwm_channel: MotorDriverPwm = pwm_channel_no_pins.output_to_pb8(pb8);
        let out1: MotorDriverOut1 = gpiob
            .pb6
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let out2: MotorDriverOut2 = gpiob
            .pb7
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

        let motor = l298n::Motor::new(out1, out2, pwm_channel);

        (motor, gpiob)
    }

    fn setup_pendulum_encoder(mut gpioe: gpioe::Parts) -> PendulumEncoder {
        let a_pe13 = gpioe
            .pe13
            .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
        let b_pe15 = gpioe
            .pe15
            .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
        let origin_offset_counts = 180;
        let counts_per_rev = 600;

        es38::Encoder::new(a_pe13, b_pe15, counts_per_rev, origin_offset_counts)
    }

    fn setup_cart_encoder(mut gpioa: gpioa::Parts) -> CartEncoder {
        let a_pa1 = gpioa
            .pa1
            .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
        let b_pa3 = gpioa
            .pa3
            .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
        let origin_offset_counts = 0;
        let counts_per_rev = 600;

        es38::Encoder::new(a_pa1, b_pa3, counts_per_rev, origin_offset_counts)

    }
}

