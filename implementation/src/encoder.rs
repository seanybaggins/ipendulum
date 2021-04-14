use embedded_hal::digital::v2::InputPin;
use hal::gpio::ExtiPin;
use stm32f3xx_hal as hal;

pub fn update<A, B>(encoder: &mut es38::Encoder<A, B>)
where
    A: ExtiPin + InputPin,
    B: ExtiPin + InputPin,
{
    defmt::trace!("encoder::update");
    encoder.hardware().pin_a().clear_interrupt_pending_bit();
    encoder.hardware().pin_b().clear_interrupt_pending_bit();
    encoder.update();
}
