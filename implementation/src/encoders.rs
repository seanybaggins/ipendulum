use core::ops::Deref;
use embedded_hal::digital::v2::InputPin;
use stm32f3xx_hal::time::Instant;

pub struct Encoder<A, B> {
    inner: rotary_encoder_hal::Rotary<A, B>,
    current_counts: u16,
    previous_counts: Option<u16>,
    prev_count_instant: Option<Instant>,
}

impl<A, B> Encoder<A, B>
where
    A: InputPin,
    B: InputPin,
{
    pub fn new(pin_a: A, pin_b: B) -> Self {
        let encoder = rotary_encoder_hal::Rotary::new(pin_a, pin_b);

        Encoder {
            inner: encoder,
            current_counts: 0,
            prev_count_instant: None,
            previous_counts: None,
        }
    }

    pub fn update() {
        
    }
}

impl<A, B> Deref for Encoder<A, B>
where
    A: InputPin,
    B: InputPin,
{
    type Target = rotary_encoder_hal::Rotary<A, B>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
