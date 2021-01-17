use std::collections::HashMap;
use std::fmt;
use derive_getters::Getters;

use crate::opcodes::*;
use crate::status_register::*;
use crate::bus::*;

#[derive(Getters)]
pub struct CPU {
    a: u8, // accumulator
    x: u8, // index register X
    y: u8, // index register Y
    pc: u16, // program counter
    sp: u16, // stack pointer
    p: StatusRegister, // processor status: [N V - B D I Z C]

    #[getter(skip)]
    bus: Bus, // memory bus
}

#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub trait Mem {
    fn mem_read_byte(&self, addr: u16) -> u8;
    fn mem_write_byte(&mut self, addr: u16, data: u8);
    
    fn mem_read_word(&self, addr: u16) -> u16 {
        let lo = self.mem_read_byte(addr);
        let hi = self.mem_read_byte(addr.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_word(&mut self, addr: u16, data: u16) {
       match data.to_le_bytes() {
           [lo, hi] => {
               self.mem_write_byte(addr, lo);
               self.mem_write_byte(addr.wrapping_add(1), hi);
           }
       }
    }
}

impl Mem for CPU {
    fn mem_read_byte(&self, addr: u16) -> u8 {
        self.bus.mem_read_byte(addr)
    }

    fn mem_write_byte(&mut self, addr: u16, data: u8) {
        self.bus.mem_write_byte(addr, data)
    }

    fn mem_read_word(&self, addr: u16) -> u16 {
        self.bus.mem_read_word(addr)
    }

    fn mem_write_word(&mut self, addr: u16, data: u16) {
        self.bus.mem_write_word(addr, data)
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CPU Dump:\n\nAccumulator: {:#02x}\nX: {:#02x}\nY: {:#02x}\nStatus: 0b{:08b}",
            self.a,
            self.x,
            self.y,
            self.p.pack()
        )
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0x1FF,
            p: StatusRegister::new(None),
            bus: Bus::new(),
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage  => self.mem_read_byte(self.pc) as u16,
            AddressingMode::Absolute => self.mem_read_word(self.pc),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read_byte(self.pc);
                pos.wrapping_add(self.x) as u16
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read_byte(self.pc);
                pos.wrapping_add(self.y) as u16
            },
            AddressingMode::Absolute_X => {
                let pos = self.mem_read_word(self.pc);
                pos.wrapping_add(self.x as u16)
            },
            AddressingMode::Absolute_Y => {
                let pos = self.mem_read_word(self.pc);
                pos.wrapping_add(self.y as u16)
            },
            AddressingMode::Indirect_X => {
                let pos = self.mem_read_byte(self.pc);
                let ptr = pos.wrapping_add(self.x);

                let lo = self.mem_read_byte(ptr as u16);
                let hi = self.mem_read_byte(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            },
            AddressingMode::Indirect_Y => {
                let pos = self.mem_read_byte(self.pc);

                let lo = self.mem_read_byte(pos as u16);
                let hi = self.mem_read_byte(pos.wrapping_add(1) as u16);
                let deref_base = u16::from_le_bytes([lo, hi]);
                deref_base.wrapping_add(self.y as u16)
            },
            AddressingMode::NoneAddressing => panic!("SIGSEGV: Invalid Addressing"),
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read_byte(addr);

        self.a = value;
        self.p.ensure_z(self.a);
        self.p.ensure_n(self.a);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write_byte(addr, self.a);
    }

    fn stack_push(&mut self, val: u8) {
        if self.sp > 0x100 {
            self.mem_write_byte(self.sp, val);
            self.sp -= 0x01;
            return;
        }

        panic!("SIGSEGV: Stack Overflow")
    }

    fn stack_push_word(&mut self, val: u16) {
        if self.sp > 0x100 {
            self.mem_write_word(self.sp - 0x01, val);
            self.sp -= 0x02;
            return;
        }

        panic!("SIGSEGV: Stack Overflow")
    }

    pub fn stack_pop(&mut self) -> u8 {
        if self.sp < 0x1FF {
            self.sp += 0x01;
            return self.mem_read_byte(self.sp);
        }

        panic!("SIGSEGV: Stack Underflow")
    }

    pub fn stack_pop_word(&mut self) -> u16 {
        if self.sp <= 0x1FE {
            self.sp += 0x02;
            return self.mem_read_word(self.sp - 0x01);
        }

        panic!("SIGSEGV: Stack Underflow")
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0x1FF;
        self.pc = self.mem_read_word(0xFFFC);
        self.p = StatusRegister::new(None);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for i in 0..(program.len() as u16) {
            self.mem_write_byte(0xC000 + i, program[i as usize]);
        }

        self.mem_write_word(0xFFFC, 0xC000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static OpCode> = *OPCODES_MAP;

        loop {
            if *self.p.B() { break; }

            let opcode = match opcodes.get(&self.mem_read_byte(self.pc)) {
                Some(x) => x,
                None => panic!("SIGILL: Unknown Instruction")
            };

            self.pc += 1;
            let pc_bak = self.pc;

            match opcode.code {
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(&opcode.mode);
                }
                
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }

                // TAX
                0xAA => {
                    self.x = self.a;
                    self.p.ensure_z(self.x);
                    self.p.ensure_n(self.x);
                },
                // TAY
                0xA8 => {
                    self.y = self.a;
                    self.p.ensure_z(self.y);
                    self.p.ensure_n(self.y);
                },
                // INX
                0xE8 => {
                    self.x = if self.x == 0xff { 1 } else { self.x + 1 };
                    self.p.ensure_z(self.x);
                    self.p.ensure_n(self.x);
                },
                // BRK
                0x00 => {
                    // Set B to generate an interrupt request
                    self.p.set_b();
                },
                // NOP
                0xEA => {},
                // PHA
                0x48 => self.stack_push(self.a),
                // PLA
                0x68 => {
                    self.a = self.stack_pop();
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },
                // JSR
                0x20 => {
                    self.stack_push_word(self.pc + 2);
                    self.pc = self.mem_read_word(self.pc);
                },
                // RTS
                0x60 => {
                    self.pc = self.stack_pop_word();
                },
                _ => panic!("SIGILL: Not Implemented")
            }

            if pc_bak == self.pc { self.pc += (opcode.len - 1) as u16; }
        }
    }
}
