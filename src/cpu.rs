use crate::instruction::*;
use crate::mem::Memory;

// 6502 CPU @ 1.79 MHz
pub struct CPU {
    pc: u16,
    sp: u8, // Stack pointer holds lowest 8 bits of next free location on the stack. The stack resides between 0x100 and 0x1FF.
    accum: i8, // Accumulator used for arithmetic operations
    x: i8,
    y: i8,
    status: StatusRegister,
    mem: Memory,
    ir: Instruction,    // currently executing instruction
    cycle: u64,         // current cycle of the processor
    pending_cycles: u8, // cycles left to execute current instruction
}

// 8-bit register that contains flags about the state of the CPU
struct StatusRegister {
    c: u8, // Carry bit: set if last operation resulted in overflow from bit 7 or underflow from bit 0
    z: u8, // Zero bit: set if result of last operation was zero
    i: u8, // Interrupt disable
    d: u8, // Decimal - no effect on NES, but on original 6502 causes some instructions to use binary-encoded decimal represenation
    b: u8, // B flag: doesn't actually exist in the status register, but exists when the flags are pushed onto the stack by certain instructions
    bit_5: u8, // Doesn't actually exist in the status register, but exists when the flags are pushed onto the stack by certain instructions
    v: u8, // Overflow: set if result of arithmetic operation resulted in invalid 2's complement result (i.e. sign bit is incorrect)
    n: u8, // Negative: contains bit 7 of value result
}

impl CPU {
    pub fn fetch_instruction(&mut self) {}
}

impl StatusRegister {
    pub fn set_c(&mut self) {
        self.c = 1;
    }
    pub fn clear_c(&mut self) {
        self.c = 0;
    }
    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn set_z(&mut self) {
        self.z = 1;
    }
    pub fn clear_z(&mut self) {
        self.z = 0;
    }
    pub fn get_z(&self) -> u8 {
        self.z
    }

    pub fn set_i(&mut self) {
        self.i = 1;
    }
    pub fn clear_i(&mut self) {
        self.i = 0;
    }
    pub fn get_i(&self) -> u8 {
        self.i
    }

    pub fn set_d(&mut self) {
        self.d = 1;
    }
    pub fn clear_d(&mut self) {
        self.d = 0;
    }
    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn set_b(&mut self) {
        self.b = 1;
    }
    pub fn clear_b(&mut self) {
        self.b = 0;
    }
    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn set_bit_5(&mut self) {
        self.bit_5 = 1;
    }
    pub fn clear_bit_5(&mut self) {
        self.bit_5 = 0;
    }
    pub fn get_bit_5(&self) -> u8 {
        self.bit_5
    }

    pub fn set_v(&mut self) {
        self.v = 1;
    }
    pub fn clear_v(&mut self) {
        self.v = 0;
    }
    pub fn get_v(&self) -> u8 {
        self.v
    }

    pub fn set_n(&mut self) {
        self.n = 1;
    }
    pub fn clear_n(&mut self) {
        self.n = 0;
    }
    pub fn get_n(&self) -> u8 {
        self.n
    }

    pub fn get_flags(&self) -> u8 {
        self.c
            | (self.z << 1)
            | (self.i << 2)
            | (self.d << 3)
            | (self.b << 4)
            | (self.bit_5 << 5)
            | (self.v << 6)
            | (self.n << 7)
    }
}
