use crate::types::{CartEncoder, MotorDriver, PendulumEncoder};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use embedded_time::{clock, duration::*, Instant};
use hal::pac::DWT;
use stm32f3xx_hal as hal;

pub const OPERATING_FREQUENCY_HZ: u32 = 16_000_000;
pub static CART_ENCODER: Mutex<RefCell<Option<CartEncoder>>> = Mutex::new(RefCell::new(None));
pub static PENDULUM_ENCODER: Mutex<RefCell<Option<PendulumEncoder>>> =
    Mutex::new(RefCell::new(None));
pub static STOPWATCH: Mutex<RefCell<Option<StopWatch>>> = Mutex::new(RefCell::new(None));
pub static MOTOR_DRIVER: Mutex<RefCell<Option<MotorDriver>>> = Mutex::new(RefCell::new(None));

// todo: arguably this should be moved to the HAL
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
