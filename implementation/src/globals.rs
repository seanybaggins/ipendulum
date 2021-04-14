use crate::types::{CartEncoder, MotorDriver, PendulumEncoder};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use embedded_time::{clock, duration::*, Instant};
use hal::pac::DWT;
use stm32f3xx_hal as hal;

pub const OPERATING_FREQUENCY_HZ: u32 = 48_000_000;
pub static mut CART_ENCODER: Option<CartEncoder> = None;
pub static mut PENDULUM_ENCODER: Option<PendulumEncoder> = None;
pub static mut STOPWATCH: Option<StopWatch> = None;
pub static mut MOTOR_DRIVER: Option<MotorDriver> = None;

pub struct StopWatch {}

impl StopWatch {
    pub fn new(mut data_watch_point_trace_unit: DWT) -> Self {
        // Now that the data watch point trace has been started,
        // it cannot be stoped
        data_watch_point_trace_unit.enable_cycle_counter();
        StopWatch {}
    }

    pub const fn max_time_milli_sec() -> u32 {
        u32::MAX / (OPERATING_FREQUENCY_HZ / 1_000)
    }
}

impl embedded_time::Clock for StopWatch {
    type T = u32;
    const SCALING_FACTOR: Fraction = Fraction::new(1, OPERATING_FREQUENCY_HZ);
    fn try_now(&self) -> Result<Instant<Self>, clock::Error> {
        // DWT::unlock();
        Ok(Instant::new(DWT::get_cycle_count()))
    }
}
