#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitField(pub u8);

impl BitField {
    #[inline(always)]
    pub fn get(&self, bit: u32) -> bool {
        self.0 & mask(bit) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, bit: u32, v: bool) {
        let mask = mask(bit);
        if v {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }
}

#[inline(always)]
fn mask(bit: u32) -> u8 {
    1 << bit
}
