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

#[rtic::app(device = stm32f3xx_hal::pac, peripherals = true, dispatchers = [EXTI9_5])]
mod app {
    use core::convert::TryInto;
    use dwt_systick_monotonic::DwtSystick;
    use implementation::{
        timing::{StopWatch, OPERATING_FREQUENCY_HZ},
        types::{CartEncoder, MotorDriver, PendulumEncoder},
    };
    use rtic::time::duration::*;
    use stm32f3xx_hal::gpio::ExtiPin;
    const LOG_PERIOD: Milliseconds = Milliseconds(100_u32);

    #[resources]
    struct Resources {
        pendulum_encoder: PendulumEncoder,
        cart_encoder: CartEncoder,
        motor_driver: MotorDriver,
    }

    #[monotonic(binds = SysTick, default = true)]
    type Timer = DwtSystick<OPERATING_FREQUENCY_HZ>;

    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        defmt::trace!("init");
        log_data::spawn_after(LOG_PERIOD).unwrap();

        let implementation::init::Hardware {
            pendulum_encoder,
            cart_encoder,
            mut motor_driver,
            mono_timer,
        } = implementation::init::Hardware::take(cx.core, cx.device);

        motor_driver.set_duty(motor_driver.get_max_duty() / 2);
        motor_driver.forward();

        (
            init::LateResources {
                pendulum_encoder,
                cart_encoder,
                motor_driver,
            },
            init::Monotonics(mono_timer),
        )
    }

    #[task(binds = EXTI0, resources = [pendulum_encoder], priority = 2)]
    fn exti0(cx: exti0::Context) {
        defmt::trace!("EXTI0");
        let mut pendulum_encoder = cx.resources.pendulum_encoder;
        pendulum_encoder.lock(|pendulum_encoder| {
            pendulum_encoder.update().expect("EXTI0 Failed");
            pendulum_encoder
                .hardware()
                .pin_a()
                .clear_interrupt_pending_bit();
        })
    }

    #[task(binds = EXTI1, resources = [cart_encoder], priority = 2)]
    fn exti1(cx: exti1::Context) {
        defmt::trace!("EXTI1");
        let mut cart_encoder = cx.resources.cart_encoder;
        cart_encoder.lock(|cart_encoder| {
            cart_encoder.update().expect("EXTI1 Failed");
            cart_encoder
                .hardware()
                .pin_a()
                .clear_interrupt_pending_bit();
        })
    }

    #[task(binds = EXTI2_TSC, resources = [pendulum_encoder], priority = 2)]
    fn exti2_tsc(cx: exti2_tsc::Context) {
        defmt::trace!("EXTI2");
        let mut pendulum_encoder = cx.resources.pendulum_encoder;
        pendulum_encoder.lock(|pendulum_encoder| {
            pendulum_encoder.update().expect("EXTI2 Failed");
            pendulum_encoder
                .hardware()
                .pin_b()
                .clear_interrupt_pending_bit();
        })
    }

    #[task(binds = EXTI3, resources = [cart_encoder], priority = 2)]
    fn exti3(cx: exti3::Context) {
        defmt::trace!("EXTI3");
        let mut cart_encoder = cx.resources.cart_encoder;
        cart_encoder.lock(|cart_encoder| {
            cart_encoder.update().expect("EXTI3 Failed");
            cart_encoder
                .hardware()
                .pin_b()
                .clear_interrupt_pending_bit();
        })
    }

    #[task(resources = [cart_encoder], priority = 1)]
    fn log_data(mut cx: log_data::Context) {
        defmt::trace!("logging");
        let duration_since_epoch_millisecs: Milliseconds = monotonics::Timer::now()
            .duration_since_epoch()
            .try_into()
            .unwrap();
        let cart_angle = cx.resources.cart_encoder.lock(|cart| cart.angle().clone());
        defmt::info!(
            "{}, {}",
            duration_since_epoch_millisecs.integer(),
            cart_angle
        );

        let over_waited_time = duration_since_epoch_millisecs % LOG_PERIOD;
        log_data::spawn_after(LOG_PERIOD - over_waited_time).unwrap();
    }
}
