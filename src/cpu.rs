use std::collections::HashMap;
use std::fmt;
use derive_getters::Getters;

use crate::opcodes::*;
use crate::status_register::*;
use crate::bus::*;

const CS_START: u16 = 0x8000;
const CS_END: u16 = 0xFFF0;

const STACK_START: u16 = 0x100;
const STACK_END: u16 = 0x1FF;

const RESET_VECTOR: u16 = 0xFFFC;
const IRQ_VECTOR: u16 = 0xFFFE;

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
            "CPU Dump:\n\nAccumulator: {:#04x}\nX: {:#04x}\nY: {:#04x}\nStatus: 0b{:08b}\nPC: {:#04x}\nSP: {:#04x}",
            self.a,
            self.x,
            self.y,
            self.p.pack(),
            self.pc,
            self.sp
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
            sp: STACK_END,
            p: StatusRegister::new(None),
            bus: Bus::new(),
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage  => self.mem_read_byte(self.pc) as u16,
            AddressingMode::Absolute => self.mem_read_word(self.pc),
            AddressingMode::Indirect => self.mem_read_word(self.mem_read_word(self.pc)),
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

    fn stack_push_byte(&mut self, val: u8) {
        if self.sp >= STACK_START {
            self.mem_write_byte(self.sp, val);
            self.sp -= 0x01;
            return;
        }

        panic!("SIGSEGV: Stack Overflow");
    }

    fn stack_push_word(&mut self, val: u16) {
        if self.sp > STACK_START {
            self.mem_write_word(self.sp - 0x01, val);
            self.sp -= 0x02;
            return;
        }

        panic!("SIGSEGV: Stack Overflow");
    }

    pub fn stack_pop_byte(&mut self) -> u8 {
        if self.sp < STACK_END {
            self.sp += 0x01;
            return self.mem_read_byte(self.sp);
        }

        panic!("SIGSEGV: Stack Underflow");
    }

    pub fn stack_pop_word(&mut self) -> u16 {
        if self.sp < STACK_END - 1 {
            self.sp += 0x02;
            return self.mem_read_word(self.sp - 0x01);
        }

        panic!("SIGSEGV: Stack Underflow");
    }

    pub fn jump_near_immediate(&mut self) {
        let offset = self.mem_read_byte(self.pc);
        if offset & 0x80 == 0x80 {
            self.pc -= 0x100 - offset as u16;
        } else {
            self.pc += offset as u16;
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = STACK_END;
        self.pc = self.mem_read_word(RESET_VECTOR);
        self.p = StatusRegister::new(None);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        if program.len() > (CS_END - CS_START) as usize {
            panic!("SIGSEGV: Unable to allocate enough memory for the program");
        }

        for i in 0..(program.len() as u16) {
            self.mem_write_byte(CS_START + i, program[i as usize]);
        }

        self.mem_write_word(RESET_VECTOR, CS_START);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F) 
    where
        F: FnMut(&mut CPU),
    {
        let ref opcodes: HashMap<u8, &'static OpCode> = *OPCODES_MAP;

        loop {
            let opcode = match opcodes.get(&self.mem_read_byte(self.pc)) {
                Some(x) => x,
                None => panic!("SIGILL: Unknown Instruction")
            };

            self.pc += 1;
            let pc_bak = self.pc;

            match opcode.code {
                // Arithmetic & logic
                // ADC
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    let carry: u8 = if *self.p.C() { 1 } else { 0 };

                    if (self.a ^ value) & 0x80 == 0x80 {
                        self.p.unset_v();
                    } else {
                        self.p.set_v();
                    }

                    let mut result: u16 = self.a as u16 + value as u16 + carry as u16;
                    self.p.ensure_z(result as u8);

                    if *self.p.D() {
                        result = (self.a & 0x0f) as u16 + (value & 0x0f) as u16 + carry as u16;
                        if result >= 0xa {
                            result = 0x10 | (result + 0x6) & 0x0f;
                        }

                        result += (self.a & 0xf0) as u16 + (value & 0xf0) as u16;
                        self.p.ensure_n(result as u8);

                        if result >= 0xa0 {
                            self.p.set_c();
                            if result >= 0x180 { self.p.unset_v(); }
                            result += 0x60;
                        } else {
                            self.p.unset_c();
                            if result < 0x80 { self.p.unset_v(); }
                        }
                    } else {
                        if result >= 0x100 {
                            self.p.set_c();
                            if result >= 0x180 { self.p.unset_v(); }
                        } else {
                            self.p.unset_c();
                            if result < 0x80 { self.p.unset_v(); }
                        }

                        self.p.ensure_n(result as u8);
                    }

                    self.a = result as u8;
                },

                // AND
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);

                    self.a &= value;
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // ASL
                0x0a | 0x06 | 0x16 | 0x0e | 0x1e => {
                    let mut value = match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_read_byte(addr)
                        }
                    };

                    if (value & 0b10000000) == 0b10000000 {
                        self.p.set_c();
                    } else {
                        self.p.unset_c();
                    }
    
                    value <<= 1;
                    match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a = value;
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_write_byte(addr, value);
                        }
                    }

                    self.p.ensure_z(value);
                    self.p.ensure_n(value);
                },

                // BIT
                0x24 | 0x2c => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    let result = value & self.a;

                    self.p.ensure_z(result);
                    self.p.ensure_n(value);

                    if value & 0x40 == 0x40 {
                        self.p.set_v();
                    } else {
                        self.p.unset_v();
                    }
                },

                // CMP
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);

                    if value <= self.a {
                        self.p.set_c();
                    } else {
                        self.p.unset_c();
                    }
 
                    self.p.ensure_z(self.a.wrapping_sub(value));
                    self.p.ensure_n(self.a.wrapping_sub(value));
                },

                // DEC
                0xc6 | 0xd6 | 0xce | 0xde => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    let result = value.wrapping_sub(1);

                    self.mem_write_byte(addr, result);
                    self.p.ensure_z(result);
                    self.p.ensure_n(result);
                },

                // EOR
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);

                    self.a ^= value;
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // LSR
                0x4a | 0x46 | 0x56 | 0x4e | 0x5e => {
                    let mut value = match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_read_byte(addr)
                        }
                    };

                    if (value & 1) == 1 {
                        self.p.set_c();
                    } else {
                        self.p.unset_c();
                    }

                    value >>= 1;
                    match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a = value;
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_write_byte(addr, value);
                        }
                    }

                    self.p.ensure_z(value);
                    self.p.ensure_n(value);
                },

                // ORA
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);

                    self.a |= value;
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // ROL
                0x2a | 0x26 | 0x36 | 0x2e | 0x3e => {
                    let value = match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_read_byte(addr)
                        }
                    };

                    let did_carry = *self.p.C();

                    if (value & 0b10000000) == 0b10000000 {
                        self.p.set_c();
                    } else {
                        self.p.unset_c();
                    }
  
                    let result = if did_carry {
                        (value << 1) | 1
                    } else {
                        value << 1
                    };

                    match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a = result;
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_write_byte(addr, result);
                        }
                    }

                    self.p.ensure_z(result);
                    self.p.ensure_n(result);
                },

                // ROR
                0x6a | 0x66 | 0x76 | 0x6e | 0x7e => {
                    let value = match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_read_byte(addr)
                        }
                    };

                    let did_carry = *self.p.C();

                    if (value & 1) == 1 {
                        self.p.set_c();
                    } else {
                        self.p.unset_c();
                    }
  
                    let result = if did_carry {
                        (value >> 1) | 0b10000000
                    } else {
                        value >> 1
                    };

                    match &opcode.mode {
                        AddressingMode::NoneAddressing => {
                            self.a = result;
                        },
                        _ => {
                            let addr = self.get_operand_address(&opcode.mode);
                            self.mem_write_byte(addr, result);
                        }
                    }

                    self.p.ensure_z(result);
                    self.p.ensure_n(result);
                },

                // SBC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    let carry: u8 = if *self.p.C() { 1 } else { 0 };
                    
                    if (self.a ^ value) & 0x80 == 0x80 {
                        self.p.set_v();
                    } else {
                        self.p.unset_v();
                    }

                    let mut result: i16 = 0; 
                    if *self.p.D() {
                        let mut low = 0x0f + (self.a & 0x0f) as i16 - (value & 0x0f) as i16 + carry as i16;
                        if low < 0x10 {
                            low -= 0x6;
                        } else {
                            result = 0x10;
                            low -= 0x10;
                        }

                        result += 0xf0 + (self.a & 0xf0) as i16 - (value & 0xf0) as i16;
                        if result < 0x100 {
                            self.p.unset_c();
                            if result < 0x80 { self.p.unset_v(); }
                            result -= 0x60;
                        } else {
                            self.p.set_c();
                            if result >= 0x180 { self.p.unset_v(); }
                        }

                        result += low;
                    } else {
                        result = 0xff + self.a as i16 - value as i16 + carry as i16;
                        if result < 0x100 {
                            self.p.unset_c();
                            if result < 0x80 { self.p.unset_v(); }
                        } else {
                            self.p.set_c();
                            if result >= 0x180 { self.p.unset_v(); }
                        }
                    }

                    self.a = result as u8;
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // Status register
                // CLC
                0x18 => self.p.unset_c(),

                // CLD
                0xd8 => self.p.unset_d(),

                // CLI
                0x58 => self.p.unset_i(),

                // CLV
                0xb8 => self.p.unset_v(),

                // SEC
                0x38 => self.p.set_c(),

                // SED
                0xf8 => self.p.set_d(),

                // SEI
                0x78 => self.p.set_i(),

                // Stack related
                // PHA
                0x48 => self.stack_push_byte(self.a),

                // PHP
                0x08 => {
                    let mut flags = self.p.clone();
                    flags.set_b();
                    flags.set_b2();
                    self.stack_push_byte(flags.pack());
                },

                // PLA
                0x68 => {
                    self.a = self.stack_pop_byte();
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // PLP
                0x28 => {
                    self.p = StatusRegister::new(Some(self.stack_pop_byte()));
                    self.p.unset_b();
                    self.p.set_b2();
                },

                // Interrupts
                // BRK
                0x00 => {
                    if !self.p.I() {
                        let handler_addr = self.mem_read_word(IRQ_VECTOR);
                        if handler_addr == 0 {
                            return;
                        }

                        self.stack_push_word(self.pc);
                        self.p.set_b();
                        self.p.set_b2();
                        self.stack_push_byte(self.p.pack());
                        self.p.set_i();
                        self.pc = handler_addr;
                    }
                },
                // RTI
                0x40 => {
                    self.p = StatusRegister::new(Some(self.stack_pop_byte()));
                    self.p.unset_b();
                    self.p.set_b2();
                    self.pc = self.stack_pop_word();
                },
                // NOP
                0xEA => {},

                // A,X,Y registers
                // CPX
                0xe0 | 0xe4 | 0xec => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    if value <= self.x {
                        self.p.set_c();
                    } else {
                        self.p.unset_c();
                    }

                    self.p.ensure_n(self.x.wrapping_sub(value));
                    self.p.ensure_z(self.x.wrapping_sub(value));
                },

                // SPY
                0xc0 | 0xc4 | 0xcc => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    if value <= self.y {
                        self.p.set_c();
                    } else {
                        self.p.unset_c();
                    }

                    self.p.ensure_n(self.y.wrapping_sub(value));
                    self.p.ensure_z(self.y.wrapping_sub(value));
                },

                // DEX
                0xca => {
                    self.x = self.x.wrapping_sub(1);
                    self.p.ensure_n(self.x);
                    self.p.ensure_z(self.x);
                },

                // DEY
                0x88 => {
                    self.y = self.y.wrapping_sub(1);
                    self.p.ensure_n(self.y);
                    self.p.ensure_z(self.y)
                },

                // INC
                0xe6 | 0xf6 | 0xee | 0xfe => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr).wrapping_add(1);
                   
                    self.mem_write_byte(addr, value);
                    self.p.ensure_z(value);
                    self.p.ensure_n(value);
                },

                // INX
                0xE8 => {
                    self.x = self.x.wrapping_add(1);
                    self.p.ensure_z(self.x);
                    self.p.ensure_n(self.x);
                },

                // INY
                0xc8 => {
                    self.y = self.y.wrapping_add(1);
                    self.p.ensure_z(self.y);
                    self.p.ensure_n(self.y);
                },

                // LDA
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);

                    self.a = value;
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // LDX
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    
                    self.x = value;
                    self.p.ensure_z(self.x);
                    self.p.ensure_n(self.x);
                },

                // LDY
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let value = self.mem_read_byte(addr);
                    
                    self.y = value;
                    self.p.ensure_z(self.y);
                    self.p.ensure_n(self.y);
                },

                // STA
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    let addr = self.get_operand_address(&opcode.mode);
                    self.mem_write_byte(addr, self.a);
                },

                // STX
                0x86 | 0x96 | 0x8e => {
                    let addr = self.get_operand_address(&opcode.mode);
                    self.mem_write_byte(addr, self.x);
                },

                // STY
                0x84 | 0x94 | 0x8c => {
                    let addr = self.get_operand_address(&opcode.mode);
                    self.mem_write_byte(addr, self.y);
                },

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

                // TSX
                0xBA => {
                    self.x = (self.sp & 0xff) as u8;
                    self.p.ensure_z(self.x);
                    self.p.ensure_n(self.x);
                },

                // TXA
                0x8a => {
                    self.a = self.x;
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // TXS
                0x9A => {
                    self.sp = STACK_START | (self.x as u16);
                },

                // TYA
                0x98 => {
                    self.a = self.y;
                    self.p.ensure_z(self.a);
                    self.p.ensure_n(self.a);
                },

                // Control flow
                // BCC
                0x90 => {
                    let condition = !*self.p.C();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // BCS
                0xb0 => {
                    let condition = *self.p.C();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // BEQ
                0xf0 => {
                    let condition = *self.p.Z();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // BMI
                0x30 => {
                    let condition = *self.p.N();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // BNE
                0xd0 => {
                    let condition = !*self.p.Z();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // BPL
                0x10 => {
                    let condition = !*self.p.N();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // BVC
                0x50 => {
                    let condition = !*self.p.V();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // BVS
                0x70 => {
                    let condition = *self.p.V();
                    if condition {
                        self.jump_near_immediate();
                    }
                },

                // JMP
                0x4c | 0x6c => {
                    self.pc = self.get_operand_address(&opcode.mode);
                },

                // JSR
                0x20 => {
                    self.stack_push_word(self.pc + 2);
                    self.pc = self.get_operand_address(&opcode.mode);
                },

                // RTS
                0x60 => {
                    self.pc = self.stack_pop_word();
                },

                _ => panic!("SIGILL: Not Implemented")
            }

            if pc_bak == self.pc { self.pc += (opcode.len - 1) as u16; }
            callback(self);
        }
    }
}
