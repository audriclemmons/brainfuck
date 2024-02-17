pub trait Value {
    const ZERO: Self;

    fn add(&mut self, n: i32);
    fn is_zero(&self) -> bool;
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
}

impl Value for u16 {
    const ZERO: Self = 0;

    #[inline]
    fn add(&mut self, n: i32) {
        *self = self.wrapping_add_signed(n as i16)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0
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
}
