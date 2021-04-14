use crate::globals::STOPWATCH;
use core::convert::TryInto;
use embedded_time::{duration::*, Clock};
pub fn get_milli_sec_since_epoch() -> Milliseconds<u32> {
    defmt::trace!("get_milli_sec_since_epoch");
    STOPWATCH
        .expect("unset stopwatch")
        .try_now()
        .unwrap()
        .duration_since_epoch()
        .try_into()
        .unwrap()
}
