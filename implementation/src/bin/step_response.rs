#![no_std]
#![no_main]
// Allows for communication back to host during panics and dubugging
// Uncomment when using GDB
// #[allow(unused_imports)]
// use cortex_m_semihosting::{dbg, hprintln};
// use panic_semihosting as _;
use panic_probe as _;

// Allows real time logging back to the host. Unfortunately, this is not compatible  with GDB
use defmt_rtt as _;

//use implementation::globals::{self, CART_ENCODER, MOTOR_DRIVER, PENDULUM_ENCODER};
//use implementation::init;
use implementation::{
    timing::StopWatch,
    types::{CartEncoder, MotorDriver, PendulumEncoder},
};

use stm32f3xx_hal::gpio::ExtiPin;

#[rtic::app(device = stm32f3xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        pendulum_encoder: PendulumEncoder,
        cart_encoder: CartEncoder,
        motor_driver: MotorDriver,
        stopwatch: StopWatch,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let implementation::init::Hardware {
            pendulum_encoder,
            cart_encoder,
            motor_driver,
            stopwatch,
        } = implementation::init::Hardware::take(cx.core, cx.device);

        init::LateResources {
            pendulum_encoder,
            cart_encoder,
            motor_driver,
            stopwatch,
        }
    }

    #[task(binds = EXTI0, resources = [pendulum_encoder], priority = 2)]
    fn exti0(cx: exti0::Context) {
        defmt::trace!("EXTI0");
        let pendulum_encoder = cx.resources.pendulum_encoder;
        pendulum_encoder.update().expect("EXTI0 Failed");
        pendulum_encoder
            .hardware()
            .pin_a()
            .clear_interrupt_pending_bit();
    }

    #[task(binds = EXTI1, resources = [cart_encoder], priority = 2)]
    fn exti1(cx: exti1::Context) {
        defmt::trace!("EXTI1");
        let cart_encoder = cx.resources.cart_encoder;
        cart_encoder.update().expect("EXTI1 Failed");
        cart_encoder
            .hardware()
            .pin_a()
            .clear_interrupt_pending_bit();
    }

    #[task(binds = EXTI2_TSC, resources = [pendulum_encoder], priority = 2)]
    fn exti2_tsc(cx: exti2_tsc::Context) {
        defmt::trace!("EXTI2");
        let pendulum_encoder = cx.resources.pendulum_encoder;
        pendulum_encoder.update().expect("EXTI2 Failed");
        pendulum_encoder
            .hardware()
            .pin_b()
            .clear_interrupt_pending_bit();
    }

    #[task(binds = EXTI3, resources = [cart_encoder], priority = 2)]
    fn exti3(cx: exti3::Context) {
        defmt::trace!("EXTI3");
        let cart_encoder = cx.resources.cart_encoder;
        cart_encoder.update().expect("EXTI3 Failed");
        cart_encoder
            .hardware()
            .pin_b()
            .clear_interrupt_pending_bit();
    }

    #[task(resources = [cart_encoder, pendulum_encoder, stopwatch], priority = 1)]
    fn log_data(mut cx: log_data::Context) {
        defmt::trace!("logging");
        let cart_angle = cx.resources.cart_encoder.lock(|cart| cart.angle().clone());
        let pendulum_angle = cx
            .resources
            .pendulum_encoder
            .lock(|pen| pen.angle().clone());
        let time_millisec = cx.resources.stopwatch.micro_seconds_since_epoch();
        defmt::info!("{}, {}, {}", time_millisec, pendulum_angle, cart_angle);
    }

    #[idle(spawn = [log_data])]
    fn idle(cx: idle::Context) -> ! {
        defmt::trace!("idle");
        cx.spawn.log_data().expect("Failed to spawn log data");
        loop {}
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn EXTI9_5();
    }
};
