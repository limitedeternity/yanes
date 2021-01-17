use crate::cpu::Mem;

#[derive(Copy, Clone)]
pub struct Bus {
    pub ram: [u8; 0xFFFF]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 0xFFFF]
        }
    }
}

impl Mem for Bus {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn mem_write_byte(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

