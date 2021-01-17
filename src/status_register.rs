#![allow(non_snake_case)]
use derive_getters::Getters;

#[derive(Getters)]
pub struct StatusRegister {
    C: bool, // carry
    Z: bool, // zero
    I: bool, // interrupt
    D: bool, // decimal
    B: bool, // break
    V: bool, // overflow
    N: bool, // negative
}

impl StatusRegister {
    pub fn new(value: Option<u16>) -> Self {
        let uvalue = value.unwrap_or(0b00000010);

        StatusRegister {
            C: (uvalue & 0b00000001) == 0b00000001,
            Z: (uvalue & 0b00000010) == 0b00000010,
            I: (uvalue & 0b00000100) == 0b00000100,
            D: (uvalue & 0b00001000) == 0b00001000,
            B: (uvalue & 0b00010000) == 0b00010000,
            V: (uvalue & 0b00100000) == 0b00100000,
            N: (uvalue & 0b01000000) == 0b01000000,
        }
    }

    pub fn ensure_z(&mut self, result: u8) {
        self.Z = result == 0;
    }

    pub fn ensure_n(&mut self, result: u8) {
        self.N = (result & 0b10000000) == 0b10000000;
    }
    
    pub fn set_b(&mut self) {
        self.B = true;
    }

    pub fn pack(&self) -> u16 {
        ((if self.C { 1 as u16 } else { 0 as u16 }) << 0) +
        ((if self.Z { 1 as u16 } else { 0 as u16 }) << 1) +
        ((if self.I { 1 as u16 } else { 0 as u16 }) << 2) +
        ((if self.D { 1 as u16 } else { 0 as u16 }) << 3) +
        ((if self.B { 1 as u16 } else { 0 as u16 }) << 4) +
        ((if self.V { 1 as u16 } else { 0 as u16 }) << 5) +
        ((if self.N { 1 as u16 } else { 0 as u16 }) << 6)
    }
}
