use crate::cpu::Mem;

pub struct Bus {
    pub ram: [u8; 0x10000]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 0x10000]
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

