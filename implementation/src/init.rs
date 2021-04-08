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
            50.Hz(),  // frequency
            &clocks,
        );
        let pwm_channel = pwm_channel_no_pins.output_to_pb8(pb8);
        let out1 = gpiob
            .pb6
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let out2 = gpiob
            .pb7
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        let motor_driver: MotorDriver = l298n::Motor::new(out1, out2, pwm_channel);

        // stopwatch creation
        let stopwatch = StopWatch::new(core_peripherals.DWT);
        let time_since_epoch_milli_sec: Milliseconds<u32> = stopwatch
            .try_now()
            .unwrap()
            .duration_since_epoch()
            .try_into()
            .unwrap();

        // Pendulum encoder setup
        let mut a_pe13 = gpioe
            .pe13
            .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
        a_pe13.make_interrupt_source(&mut syscfg);
        a_pe13.trigger_on_edge(&mut exti, Edge::Rising);
        a_pe13.enable_interrupt(&mut exti);

        let b_pe15 = gpioe
            .pe15
            .into_pull_up_input(&mut gpioe.moder, &mut gpioe.pupdr);
        let origin_offset_counts = 180;
        let counts_per_rev = 2400;

        let initial_angle = es38::Angle::new(counts_per_rev, origin_offset_counts);
        let pendulum_encoder: PendulumEncoder =
            es38::Encoder::new(a_pe13, b_pe15, initial_angle, time_since_epoch_milli_sec);

        // Cart encoder setup
        let mut a_pa1 = gpioa
            .pa1
            .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
        a_pa1.make_interrupt_source(&mut syscfg);
        a_pa1.trigger_on_edge(&mut exti, Edge::RisingFalling);
        a_pa1.enable_interrupt(&mut exti);

        let mut b_pa3 = gpioa
            .pa3
            .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
        b_pa3.make_interrupt_source(&mut syscfg);
        b_pa3.trigger_on_edge(&mut exti, Edge::RisingFalling);
        b_pa3.enable_interrupt(&mut exti);

        let origin_offset_counts = 0;
        let counts_per_rev = 2400;

        let initial_angle = es38::Angle::new(counts_per_rev, origin_offset_counts);
        let cart_encoder: CartEncoder =
            es38::Encoder::new(a_pa1, b_pa3, initial_angle, time_since_epoch_milli_sec);

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
    interrupt_free(|cs| {
        CART_ENCODER.borrow(cs).replace(Some(cart_encoder));
        PENDULUM_ENCODER.borrow(cs).replace(Some(pendulum_encoder));
        STOPWATCH.borrow(cs).replace(Some(stopwatch));
        MOTOR_DRIVER.borrow(cs).replace(Some(motor_driver));
    });

    defmt::debug!("Unmasking interrupts");
    unsafe {
        NVIC::unmask(Interrupt::EXTI1);
        NVIC::unmask(Interrupt::EXTI3);
        //NVIC::unmask(Interrupt::EXTI15_10);
    }
}
