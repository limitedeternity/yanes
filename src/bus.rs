macro_rules! box_array {
    ($val:expr; $len:expr) => {{
        fn box_from_vec<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();
            let ptr = Box::into_raw(boxed_slice) as *mut [T; $len];
            unsafe { Box::from_raw(ptr) }
        }

        box_from_vec(vec![$val; $len])
    }};
}

struct OperativeMemory {
    ram: Box<[u8; u16::MAX as usize + 1]>,
}

impl OperativeMemory {
    fn new() -> Self {
        OperativeMemory {
            ram: box_array![0; u16::MAX as usize + 1],
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

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
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
