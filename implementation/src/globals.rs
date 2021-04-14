use hal::pac::DWT;
use stm32f3xx_hal as hal;

pub const OPERATING_FREQUENCY_HZ: u32 = 48_000_000;

pub struct StopWatch {
    last_time_since_the_epoch_milli_sec: u32,
}

impl StopWatch {
    pub fn new(mut data_watch_point_trace_unit: DWT) -> Self {
        // Now that the data watch point trace has been started,
        // it cannot be stoped
        data_watch_point_trace_unit.enable_cycle_counter();
        StopWatch {
            last_time_since_the_epoch_milli_sec: DWT::get_cycle_count()
                / (OPERATING_FREQUENCY_HZ / 1_000),
        }
    }

    fn get_cycle_count(&self) -> u32 {
        DWT::get_cycle_count()
    }

    pub const fn dwt_max_time_milli_sec() -> u32 {
        u32::MAX / (OPERATING_FREQUENCY_HZ / 1_000)
    }

    pub fn milli_sec_since_epoch(&mut self) -> u32 {
        let current_time_since_epoch_milli_sec =
            self.get_cycle_count() / (OPERATING_FREQUENCY_HZ / 1_000);
        if current_time_since_epoch_milli_sec > self.last_time_since_the_epoch_milli_sec {
            self.last_time_since_the_epoch_milli_sec = current_time_since_epoch_milli_sec;
        } else if current_time_since_epoch_milli_sec < self.last_time_since_the_epoch_milli_sec {
            // Then arthimetic overflow of the dwt occured. Correct it.
            self.last_time_since_the_epoch_milli_sec =
                Self::dwt_max_time_milli_sec() + current_time_since_epoch_milli_sec;
        } else {
            panic!("Data watch point peripheral returned the same value twice");
        }

        self.last_time_since_the_epoch_milli_sec
    }
}
