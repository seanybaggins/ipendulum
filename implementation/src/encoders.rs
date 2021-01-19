use embedded_hal::digital::v2::InputPin;
use stm32f3xx_hal::time::Instant;
use core::ops::Deref;

pub struct PendulumEncoder<A, B> {
    inner: rotary_encoder_hal::Rotary<A, B>,
    current_counts: u16,
    previous_counts: Option<u16>,
    prev_count_instant: Option<Instant>
}

impl<A, B> PendulumEncoder<A, B>
where
    A: InputPin,
    B: InputPin,
{
    pub fn new(pin_a: A, pin_b: B) -> Self {
        let encoder = rotary_encoder_hal::Rotary::new(pin_a, pin_b);

        PendulumEncoder {
            inner: encoder,
            prev_count_instant: None,
            current_counts: 0,
            previous_counts: None
        }
    }
}

impl<A, B> Deref for PendulumEncoder<A, B>
where
    A: InputPin,
    B: InputPin,
{
    type Target = rotary_encoder_hal::Rotary<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}