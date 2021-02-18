use hal::{gpio::ExtiPin, prelude::*};
use hal::{
    gpio::{
        gpioa::{PA1, PA3},
        gpiob::{PB6, PB7, PB8},
        gpioe::{PE13, PE15},
        Edge,
    },
    gpio::{Input, Output, PullUp, PushPull, AF1},
    pac, pwm,
    pwm::{PwmChannel, WithPins, TIM16_CH1},
};
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
        let mut syscfg = device_peripherals.SYSCFG;
        let mut exti = device_peripherals.EXTI;

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

        // Motor setup
        let pb8: MotorDriverPwmPin = gpiob.pb8.into_af1(&mut gpiob.moder, &mut gpiob.afrh);
        let pwm_channel_no_pins = pwm::tim16(
            device_peripherals.TIM16,
            u16::MAX, // resolution
            50.hz(),  // frequency
            &clocks,
        );
        let pwm_channel: MotorDriverPwm = pwm_channel_no_pins.output_to_pb8(pb8);
        let out1 = gpiob
            .pb6
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let out2 = gpiob
            .pb7
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let motor_driver: MotorDriver = l298n::Motor::new(out1, out2, pwm_channel);

        // Pendulum encoder setup
        let mut a_pe13: PendulumEncIn1 = gpioe
            .pe13
            .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
        a_pe13.make_interrupt_source(&mut syscfg);
        a_pe13.trigger_on_edge(&mut exti, Edge::Rising);
        a_pe13.enable_interrupt(&mut exti);

        let b_pe15: PendulumEncIn2 = gpioe
            .pe15
            .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
        let origin_offset_counts = 180;
        let counts_per_rev = 600;

        let pendulum_encoder: PendulumEncoder =
            es38::Encoder::new(a_pe13, b_pe15, counts_per_rev, origin_offset_counts);

        // Cart encoder setup
        let mut a_pa1: CartEncIn1 = gpioa
            .pa1
            .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
        a_pa1.make_interrupt_source(&mut syscfg);
        a_pa1.trigger_on_edge(&mut exti, Edge::Rising);
        a_pa1.enable_interrupt(&mut exti);

        let b_pa3: CartEncIn2 = gpioa
            .pa3
            .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
        let origin_offset_counts = 0;
        let counts_per_rev = 600;

        let cart_encoder: CartEncoder =
            es38::Encoder::new(a_pa1, b_pa3, counts_per_rev, origin_offset_counts);

        Hardware {
            motor_driver,
            pendulum_encoder,
            cart_encoder,
        }
    }
}
