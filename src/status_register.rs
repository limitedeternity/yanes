#![allow(non_snake_case)]
use derive_getters::Getters;

#[derive(Getters, Clone)]
pub struct StatusRegister {
    C: bool, // carry
    Z: bool, // zero
    I: bool, // interrupt
    D: bool, // decimal
    B: bool, // break
    B2: bool, // break2
    V: bool, // overflow
    N: bool, // negative
}

impl StatusRegister {
    pub fn new(value: Option<u8>) -> Self {
        let uvalue = value.unwrap_or(0b00000010);

        StatusRegister {
            C: (uvalue & 0b00000001) == 0b00000001,
            Z: (uvalue & 0b00000010) == 0b00000010,
            I: (uvalue & 0b00000100) == 0b00000100,
            D: (uvalue & 0b00001000) == 0b00001000,
            B: (uvalue & 0b00010000) == 0b00010000,
            B2: (uvalue & 0b00100000) == 0b00100000,
            V: (uvalue & 0b01000000) == 0b01000000,
            N: (uvalue & 0b10000000) == 0b10000000,
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
        self.N = (result & 0b10000000) == 0b10000000;
    }

    pub fn pack(&self) -> u8 {
        ((if self.C  { 1u8 } else { 0u8 }) << 0) +
        ((if self.Z  { 1u8 } else { 0u8 }) << 1) +
        ((if self.I  { 1u8 } else { 0u8 }) << 2) +
        ((if self.D  { 1u8 } else { 0u8 }) << 3) +
        ((if self.B  { 1u8 } else { 0u8 }) << 4) +
        ((if self.B2 { 1u8 } else { 0u8 }) << 5) +
        ((if self.V  { 1u8 } else { 0u8 }) << 6) +
        ((if self.N  { 1u8 } else { 0u8 }) << 7)
    }
}
