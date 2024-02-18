pub trait Value {
    const ZERO: Self;

    fn add(&mut self, n: i32);
    fn is_zero(&self) -> bool;

    fn get_byte(&self) -> u8;
    fn set_byte(&mut self, byte: u8);
}

impl Value for u32 {
    const ZERO: Self = 0;

    #[inline]
    fn add(&mut self, n: i32) {
        *self = self.wrapping_add_signed(n as i32)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }

    #[inline]
    fn get_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }

    #[inline]
    fn set_byte(&mut self, byte: u8) {
        *self = byte as Self
    }
}

impl Value for u8 {
    const ZERO: Self = 0;

    #[inline]
    fn add(&mut self, n: i32) {
        *self = self.wrapping_add_signed(n as i8)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
    }

    #[inline]
    fn get_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }

    #[inline]
    fn set_byte(&mut self, byte: u8) {
        *self = byte as Self
    }
}
