use crate::globals::{
    StopWatch, CART_ENCODER, MOTOR_DRIVER, OPERATING_FREQUENCY_HZ, PENDULUM_ENCODER, STOPWATCH,
};
use crate::types::{CartEncoder, MotorDriver, MotorDriverPwmPin, PendulumEncoder};
use core::convert::TryInto;
use cortex_m::interrupt::free as interrupt_free;
use embedded_time::Clock;
use hal::{
    gpio::{Edge, ExtiPin},
    pac::{self, Interrupt, NVIC},
    prelude::*,
    pwm,
    time::{duration::*, rate::*},
};
use stm32f3xx_hal as hal;

pub struct Hardware {
    pub pendulum_encoder: PendulumEncoder,
    pub cart_encoder: CartEncoder,
    pub motor_driver: MotorDriver,
    pub stopwatch: StopWatch,
}

impl Hardware {
    pub fn take() -> Self {
        defmt::trace!("Hardware take");
        // Typical acquiring of board singleton and setting the clock speed
        let core_peripherals = pac::CorePeripherals::take().unwrap();
        let device_peripherals = pac::Peripherals::take().unwrap();
        let mut flash = device_peripherals.FLASH.constrain();
        let mut reset_and_control_clock = device_peripherals.RCC.constrain();
        let mut syscfg = device_peripherals.SYSCFG;
        let mut exti = device_peripherals.EXTI;

        let clocks = reset_and_control_clock
            .cfgr
            .sysclk(OPERATING_FREQUENCY_HZ.Hz())
            .expect("Failed to set frequency of systemctl clock")
            .freeze(&mut flash.acr);

        // Pin ports in use
        let mut gpiod = device_peripherals
            .GPIOD
            .split(&mut reset_and_control_clock.ahb);
        let mut gpioa = device_peripherals
            .GPIOA
            .split(&mut reset_and_control_clock.ahb);

        // Motor setup
        let motor_driver_pwm_pin: MotorDriverPwmPin =
            gpiod.pd1.into_af4(&mut gpiod.moder, &mut gpiod.afrl);
        let pwm_channel_no_pins = pwm::tim8(
            device_peripherals.TIM8,
            u16::MAX, // resolution
            50.Hz(),  // frequency
            &clocks,
        );
        let pwm_channel = pwm_channel_no_pins.3.output_to_pd1(motor_driver_pwm_pin);
        let out1 = gpiod
            .pd3
            .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
        let out2 = gpiod
            .pd4
            .into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
        let motor_driver: MotorDriver = l298n::Motor::new(out1, out2, pwm_channel);

        // stopwatch creation
        let stopwatch = StopWatch::new(core_peripherals.DWT);

        // Pendulum encoder setup
        let mut pendulum_encoder_in_1_pin = gpioa
            .pa0
            .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);
        pendulum_encoder_in_1_pin.make_interrupt_source(&mut syscfg);
        pendulum_encoder_in_1_pin.trigger_on_edge(&mut exti, Edge::RisingFalling);
        pendulum_encoder_in_1_pin.enable_interrupt(&mut exti);

        let mut pendulum_encoder_in_2_pin = gpioa
            .pa2
            .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);
        pendulum_encoder_in_2_pin.make_interrupt_source(&mut syscfg);
        pendulum_encoder_in_2_pin.trigger_on_edge(&mut exti, Edge::RisingFalling);
        pendulum_encoder_in_2_pin.enable_interrupt(&mut exti);

        let origin_offset_counts = 0;
        let counts_per_rev = 2400;

        let initial_angle = es38::Angle::new(counts_per_rev, origin_offset_counts);
        let pendulum_encoder: PendulumEncoder = es38::Encoder::new(
            pendulum_encoder_in_1_pin,
            pendulum_encoder_in_2_pin,
            initial_angle,
        );

        // Cart encoder setup
        let mut cart_encoder_in_1_pin = gpioa
            .pa1
            .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);
        cart_encoder_in_1_pin.make_interrupt_source(&mut syscfg);
        cart_encoder_in_1_pin.trigger_on_edge(&mut exti, Edge::RisingFalling);
        cart_encoder_in_1_pin.enable_interrupt(&mut exti);

        let mut cart_encoder_in_2_pin = gpioa
            .pa3
            .into_floating_input(&mut gpioa.moder, &mut gpioa.pupdr);
        cart_encoder_in_2_pin.make_interrupt_source(&mut syscfg);
        cart_encoder_in_2_pin.trigger_on_edge(&mut exti, Edge::RisingFalling);
        cart_encoder_in_2_pin.enable_interrupt(&mut exti);

        let origin_offset_counts = 0;
        let counts_per_rev = 2400;

        let initial_angle = es38::Angle::new(counts_per_rev, origin_offset_counts);
        let cart_encoder: CartEncoder =
            es38::Encoder::new(cart_encoder_in_1_pin, cart_encoder_in_2_pin, initial_angle);

        Hardware {
            motor_driver,
            pendulum_encoder,
            cart_encoder,
            stopwatch,
        }
    }
}

pub fn setup() {
    defmt::trace!("main");
    let Hardware {
        pendulum_encoder,
        cart_encoder,
        motor_driver,
        stopwatch,
    } = Hardware::take();

    // handing the hardware over to a global context do they can be accessed within an interrupt
    CART_ENCODER = Some(cart_encoder);
    PENDULUM_ENCODER = Some(pendulum_encoder);
    STOPWATCH = Some(stopwatch);
    MOTOR_DRIVER = Some(motor_driver);

    defmt::debug!("Unmasking interrupts");
    unsafe {
        NVIC::unmask(Interrupt::EXTI0);
        NVIC::unmask(Interrupt::EXTI1);
        NVIC::unmask(Interrupt::EXTI2_TSC);
        NVIC::unmask(Interrupt::EXTI3);
    }
}
