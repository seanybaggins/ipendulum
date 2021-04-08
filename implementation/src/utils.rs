use crate::globals::STOPWATCH;
use core::convert::TryInto;
use cortex_m::interrupt::CriticalSection;
use embedded_time::Clock;
use hal::time::duration::*;
use stm32f3xx_hal as hal;

#[macro_export]
macro_rules! get_global_ref {
    ($global_item:ident, $cs:ident) => {
        $global_item
            .borrow($cs)
            .borrow()
            .as_ref()
            .expect("failed to get global reference")
    };
}

#[macro_export]
macro_rules! get_global_ref_mut {
    ($global_item:ident, $cs:ident) => {
        $global_item
            .borrow($cs)
            .borrow_mut()
            .as_mut()
            .expect("failed to get global mutible reference")
    };
}

#[macro_export]
macro_rules! update_encoder {
    ($encoder:ident, $cs:ident) => {{
        use hal::gpio::ExtiPin;
        defmt::trace!("encoder update");

        let millisec_since_epoch = crate::utils::get_milli_sec_since_epoch($cs);

        // Update the angle and direction state of the encoder
        crate::get_global_ref_mut!($encoder, $cs)
            .update(millisec_since_epoch)
            .unwrap();
        crate::get_global_ref_mut!($encoder, $cs)
            .hardware()
            .pin_a()
            .clear_interrupt_pending_bit();
        crate::get_global_ref_mut!($encoder, $cs)
            .hardware()
            .pin_b()
            .clear_interrupt_pending_bit();
    }};
}

pub fn get_milli_sec_since_epoch(cs: &CriticalSection) -> Milliseconds<u32> {
    defmt::trace!("get_milli_sec_since_epoch");
    get_global_ref!(STOPWATCH, cs)
        .try_now()
        .unwrap()
        .duration_since_epoch()
        .try_into()
        .unwrap()
}
