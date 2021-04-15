use hal::pac::DWT;
use stm32f3xx_hal as hal;
//use time::duration::*;

pub const OPERATING_FREQUENCY_HZ: u32 = 48_000_000;

pub struct StopWatch {
    cycle_minute_hand: u32,
    cycle_hour_hand: u32,
}

impl StopWatch {
    pub fn new(mut data_watch_point_trace_unit: DWT) -> Self {
        // Now that the data watch point trace has been started,
        // it cannot be stoped
        data_watch_point_trace_unit.enable_cycle_counter();
        StopWatch {
            cycle_minute_hand: DWT::get_cycle_count(),
            cycle_hour_hand: 0,
        }
    }

    fn get_cycle_count(&self) -> u32 {
        DWT::get_cycle_count()
    }

    fn update(&mut self) {
        let current_cylce_minute_hand = self.get_cycle_count();
        if current_cylce_minute_hand > self.cycle_minute_hand {
            self.cycle_minute_hand = current_cylce_minute_hand;
        } else if current_cylce_minute_hand < self.cycle_minute_hand {
            self.cycle_minute_hand = current_cylce_minute_hand;
            self.cycle_hour_hand += 1;
        } else {
            panic!("DWT returned the same value twice");
        }
    }

    fn cycles_since_epoch(&mut self) -> u64 {
        self.update();
        let mut cycles_since_epoch: u64 = self.cycle_hour_hand as u64;
        cycles_since_epoch = cycles_since_epoch << 32;
        cycles_since_epoch += self.cycle_minute_hand as u64;

        cycles_since_epoch
    }

    pub fn micro_seconds_since_epoch(&mut self) -> u64 {
        self.cycles_since_epoch() / (OPERATING_FREQUENCY_HZ / 1_000_000) as u64
    }
}
