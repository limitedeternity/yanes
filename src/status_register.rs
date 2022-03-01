#![allow(non_snake_case)]
use derive_getters::Getters;

#[derive(Getters, Clone)]
pub struct StatusRegister {
    C: bool,  // carry
    Z: bool,  // zero
    I: bool,  // interrupt
    D: bool,  // decimal
    B: bool,  // break
    B2: bool, // break2
    V: bool,  // overflow
    N: bool,  // negative
}

impl Default for StatusRegister {
    fn default() -> Self {
        Self::new(0b0000_0010)
    }
}

impl StatusRegister {
    pub fn new(value: u8) -> Self {
        StatusRegister {
            C: (value & 0b0000_0001) == 0b0000_0001,
            Z: (value & 0b0000_0010) == 0b0000_0010,
            I: (value & 0b0000_0100) == 0b0000_0100,
            D: (value & 0b0000_1000) == 0b0000_1000,
            B: (value & 0b0001_0000) == 0b0001_0000,
            B2: (value & 0b0010_0000) == 0b0010_0000,
            V: (value & 0b0100_0000) == 0b0100_0000,
            N: (value & 0b1000_0000) == 0b1000_0000,
        }
    }

    pub fn set_c(&mut self) {
        self.C = true;
    }

    pub fn unset_c(&mut self) {
        self.C = false;
    }

    pub fn set_i(&mut self) {
        self.I = true;
    }

    pub fn unset_i(&mut self) {
        self.I = false;
    }

    pub fn set_d(&mut self) {
        self.D = true;
    }

    pub fn unset_d(&mut self) {
        self.D = false;
    }

    pub fn set_b(&mut self) {
        self.B = true;
    }

    pub fn unset_b(&mut self) {
        self.B = false;
    }

    pub fn set_b2(&mut self) {
        self.B2 = true;
    }

    pub fn unset_b2(&mut self) {
        self.B2 = false;
    }

    pub fn set_v(&mut self) {
        self.V = true;
    }

    pub fn unset_v(&mut self) {
        self.V = false;
    }

    pub fn ensure_z(&mut self, result: u8) {
        self.Z = result == 0;
    }

    pub fn ensure_n(&mut self, result: u8) {
        self.N = (result & 0b1000_0000) == 0b1000_0000;
    }

    pub fn pack(&self) -> u8 {
        (if self.C { 1u8 } else { 0u8 })
            + ((if self.Z { 1u8 } else { 0u8 }) << 1)
            + ((if self.I { 1u8 } else { 0u8 }) << 2)
            + ((if self.D { 1u8 } else { 0u8 }) << 3)
            + ((if self.B { 1u8 } else { 0u8 }) << 4)
            + ((if self.B2 { 1u8 } else { 0u8 }) << 5)
            + ((if self.V { 1u8 } else { 0u8 }) << 6)
            + ((if self.N { 1u8 } else { 0u8 }) << 7)
    }
}
