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

pub trait Mem {
    fn mem_read_byte(&self, addr: u16) -> u8;
    fn mem_write_byte(&mut self, addr: u16, data: u8);

    fn mem_read_word(&self, addr: u16) -> u16 {
        let lo = self.mem_read_byte(addr);
        let hi = self.mem_read_byte(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_word(&mut self, addr: u16, data: u16) {
       match data.to_le_bytes() {
           [lo, hi] => {
               self.mem_write_byte(addr, lo);
               self.mem_write_byte(addr + 1, hi);
           }
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

