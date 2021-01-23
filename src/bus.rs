struct OperativeMemory {
    ram: [u8; 0x10000]
}

impl OperativeMemory {
    fn new() -> Self {
        OperativeMemory {
            ram: [0; 0x10000]
        }
    }
}

trait StorageDevice {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, data: u8);

    fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr);
        let hi = self.read_byte(addr + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn write_word(&mut self, addr: u16, data: u16) {
       match data.to_le_bytes() {
           [lo, hi] => {
               self.write_byte(addr, lo);
               self.write_byte(addr + 1, hi);
           }
       }
    }
}

impl StorageDevice for OperativeMemory {
    fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

pub struct Bus {
    operative_memory: OperativeMemory,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            operative_memory: OperativeMemory::new(),
        }
    }

    pub fn mem_read_byte(&self, addr: u16) -> u8 {
        self.operative_memory.read_byte(addr)
    }

    pub fn mem_write_byte(&mut self, addr: u16, data: u8) {
        self.operative_memory.write_byte(addr, data);
    }

    pub fn mem_read_word(&self, addr: u16) -> u16 {
        self.operative_memory.read_word(addr)
    }

    pub fn mem_write_word(&mut self, addr: u16, data: u16) {
        self.operative_memory.write_word(addr, data); 
    }
}
