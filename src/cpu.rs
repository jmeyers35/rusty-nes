use crate::instruction::*;
use crate::mem;
use crate::mem::Memory;
use std::fmt;

// 6502 CPU @ 1.79 MHz
pub struct CPU {
    pc: u16,
    sp: u8, // Stack pointer holds lowest 8 bits of next free location on the stack. The stack resides between 0x100 and 0x1FF.
    accum: i8, // Accumulator used for arithmetic operations
    x: i8,
    y: i8,
    status: StatusRegister,
    mem: Memory,
    cycle: u64, // current cycle of the processor
}

// 8-bit register that contains flags about the state of the CPU
#[derive(Debug)]
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
    pub fn new() -> CPU {
        CPU {
            pc: 0,
            sp: 0xfd,
            accum: 0,
            x: 0,
            y: 0,
            status: StatusRegister::new(),
            mem: Memory::new(),
            cycle: 0,
        }
    }
    fn tick_clock(&mut self) {
        // Later: This will also tick the PPU * 3
        self.cycle += 1;
    }
    fn fetch_instruction(&mut self) -> Instruction {
        let opcode = self.mem.read(self.pc);
        self.pc += 1;
        Instruction::new(opcode)
    }
    fn push_byte(&mut self, data: u8) {
        self.mem.write(mem::STACK_TOP + self.sp as u16, data);
        self.sp -= 1;
    }
    fn pop_byte(&mut self) -> u8 {
        self.sp += 1;
        self.mem.read(mem::STACK_TOP + self.sp as u16)
    }

    fn execute_instruction(&mut self, inst: &Instruction) {
        match inst.op {
            // TODO validate carry flag and overflow flag behavior
            OpCode::ADC => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if signed_overflow_add(self.accum, operand + self.status.get_c() as i8) {
                    self.status.set_v();
                } else {
                    self.status.clear_v();
                }
                if self
                    .accum
                    .checked_add(operand + self.status.get_c() as i8)
                    .is_none()
                {
                    self.status.set_c();
                } else {
                    self.status.clear_c();
                }
                self.accum = self.accum.wrapping_add(operand + self.status.get_c() as i8);
            }
            OpCode::AND => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                self.accum &= operand;
                if self.accum == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.accum < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::ASL => {
                let (operand, addr) = self.get_operand(inst.addr_mode);
                if (operand & (1 << 7)) >> 7 == 1 {
                    self.status.set_c();
                } else {
                    self.status.clear_c();
                }
                let result = operand << 1;
                if (result & (1 << 7)) >> 7 == 1 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
                if let Some(addr) = addr {
                    self.mem.write(addr, result as u8);
                } else {
                    self.accum = result;
                }
            }
            OpCode::BCC => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_c() == 0 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::BCS => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_c() == 1 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::BEQ => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_z() == 1 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::BIT => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                let result = self.accum & operand;
                if result == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if (result & (1 << 6)) >> 6 == 1 {
                    self.status.set_v();
                } else {
                    self.status.clear_v();
                }
                if (result & (1 << 7)) >> 7 == 1 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::BMI => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_n() == 1 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::BNE => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_z() == 0 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::BPL => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_n() == 0 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::BRK => {
                // Step 1: Push PC and Status Flags onto the stack
                let pc_lsb = (self.pc & 0xFF) as u8;
                let pc_msb = ((self.pc & 0xFF00) >> 8) as u8;

                self.push_byte(pc_lsb);
                self.push_byte(pc_msb);
                self.push_byte(self.status.get_flags());

                // Step 2: Load IRQ vector (held at 0xFFFE and OXFFFF) into PC
                let irq_vec_lsb = self.mem.read(0xFFFE) as u16;
                let irq_vec_msb = self.mem.read(0xFFFF) as u16;
                self.pc = irq_vec_lsb | (irq_vec_msb << 8);

                // Step 3: Set B flag
                self.status.set_b();
            }
            OpCode::BVC => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_v() == 0 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::BVS => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.status.get_v() == 1 {
                    let new_pc = (self.pc as i16 + operand as i16) as u16;
                    if page_crossed(self.pc, new_pc) {
                        self.tick_clock();
                    }
                    self.pc = (self.pc as i16 + operand as i16) as u16;
                    self.tick_clock();
                }
            }
            OpCode::CLC => {
                self.status.clear_c();
            }
            OpCode::CLD => {
                self.status.clear_d();
            }
            OpCode::CLI => {
                self.status.clear_i();
            }
            OpCode::CLV => {
                self.status.clear_v();
            }
            OpCode::CMP => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.accum >= operand {
                    self.status.set_c();
                } else {
                    self.status.clear_c();
                }
                if self.accum == operand {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.accum - operand < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::CPX => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.x >= operand {
                    self.status.set_c();
                } else {
                    self.status.clear_c();
                }
                if self.x == operand {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.x - operand < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::CPY => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if self.y >= operand {
                    self.status.set_c();
                } else {
                    self.status.clear_c();
                }
                if self.y == operand {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.y - operand < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::DEC => {
                let (operand, addr) = self.get_operand(inst.addr_mode);
                let res = operand - 1;
                if res == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if res < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
                self.mem.write(addr.unwrap(), res as u8);
            }
            OpCode::DEX => {
                self.x -= 1;
                if self.x == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.x < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::DEY => {
                self.y -= 1;
                if self.y == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.y < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::EOR => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                self.accum ^= operand;
                if self.accum == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.accum < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::INC => {
                let (operand, addr) = self.get_operand(inst.addr_mode);
                let res = operand + 1;
                if res == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if res < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
                self.mem.write(addr.unwrap(), res as u8);
            }
            OpCode::INX => {
                self.x += 1;
                if self.x == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.x < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::INY => {
                self.y += 1;
                if self.y == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.y < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::JMP => {
                let (_, addr) = self.get_operand(inst.addr_mode);
                self.pc = addr.unwrap();
            }
            OpCode::JSR => {
                let (_, addr) = self.get_operand(inst.addr_mode);
                // Push ret addr onto stack
                let pc_lsb = (self.pc & 0xFF) as u8;
                let pc_msb = ((self.pc & 0xFF00) >> 8) as u8;
                self.push_byte(pc_lsb);
                self.push_byte(pc_msb);
                self.pc = addr.unwrap();
            }
            OpCode::LDA => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                self.accum = operand as i8;
                if self.accum == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.accum < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::LDX => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                self.x = operand as i8;
                if self.x == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.x < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::LDY => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                self.y = operand as i8;
                if self.y == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.y < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::LSR => {
                let (operand, addr) = self.get_operand(inst.addr_mode);
                let sign_bit = operand & (1 << 7);
                if sign_bit == 1 {
                    self.status.set_c();
                } else {
                    self.status.clear_c();
                }
                // Have to do this bc behavior of shift operator in Rust depends on signed-ness of operand (arithmetic shift on i-types, logical shift on u-types)
                let result = (operand as u8) >> 1;
                self.status.clear_n();
                if let Some(addr) = addr {
                    self.mem.write(addr, result);
                } else {
                    self.accum = result as i8;
                }
            }
            OpCode::NOP => {}
            OpCode::ORA => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                self.accum |= operand;
                if self.accum == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.accum < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::PHA => {
                self.push_byte(self.accum as u8);
            }
            OpCode::PHP => {
                self.push_byte(self.status.get_flags());
            }
            OpCode::PLA => {
                self.accum = self.pop_byte() as i8;
                if self.accum == 0 {
                    self.status.set_z();
                } else {
                    self.status.clear_z();
                }
                if self.accum < 0 {
                    self.status.set_n();
                } else {
                    self.status.clear_n();
                }
            }
            OpCode::PLP => {
                let flags = self.pop_byte();
                self.status.set_flags(flags);
            }
            OpCode::ROL => {
                // Semantics: Bit 0 is filled with current carry flag value. Old bit 7 goes into the carry flag.
                let (operand, addr) = self.get_operand(inst.addr_mode);
                let old_sign_bit = operand & (1 << 7);
                let curr_carry_flag = self.status.get_c();
                if old_sign_bit == 0 {
                    self.status.clear_c();
                } else {
                    self.status.set_c();
                }
                let mut result = operand.rotate_left(1);
                // clear least significant bit
                result &= !1;
                result |= curr_carry_flag as i8;
                if let Some(addr) = addr {
                    self.mem.write(addr, result as u8);
                } else {
                    self.accum = result;
                }
            }
            OpCode::ROR => {
                // Semantics: Bit 7 is filled with the current carry flag value. Old bit 0 goes into the carry flag.
                let (operand, addr) = self.get_operand(inst.addr_mode);
                let old_bit_zero = operand & 1;
                let curr_carry_flag = self.status.get_c();
                if old_bit_zero == 0 {
                    self.status.clear_c();
                } else {
                    self.status.set_c();
                }
                let mut result = operand.rotate_right(1);
                // clear msb
                result &= !(1 << 7);
                result |= (curr_carry_flag << 7) as i8;
                if let Some(addr) = addr {
                    self.mem.write(addr, result as u8);
                } else {
                    self.accum = result;
                }
            }
            OpCode::RTI => {
                let new_flags = self.pop_byte();
                self.status.set_flags(new_flags);
                let pc_msb = self.pop_byte();
                let pc_lsb = self.pop_byte();
                let new_pc = pc_lsb as u16 | (pc_msb as u16) << 8;
                self.pc = new_pc;
            }
            OpCode::RTS => {
                let pc_msb = self.pop_byte();
                let pc_lsb = self.pop_byte();
                let new_pc = pc_lsb as u16 | (pc_msb as u16) << 8;
                self.pc = new_pc;
            }
            // TODO validate carry flag and overflow flag behavior
            OpCode::SBC => {
                let (operand, _) = self.get_operand(inst.addr_mode);
                if signed_overflow_sub(self.accum, operand - (1 - self.status.get_c()) as i8) {
                    self.status.set_v();
                } else {
                    self.status.clear_v();
                }
                if self
                    .accum
                    .checked_add(operand - (1 - self.status.get_c()) as i8)
                    .is_none()
                {
                    self.status.set_c();
                } else {
                    self.status.clear_c();
                }
                self.accum = self.accum.wrapping_sub(operand + self.status.get_c() as i8);
            }
            OpCode::SEC => {
                self.status.set_c();
            }
            OpCode::SED => {
                self.status.set_d();
            }
            OpCode::SEI => {
                self.status.set_i();
            }
            OpCode::STA => {
                let (_, addr) = self.get_operand(inst.addr_mode);
                self.mem.write(addr.unwrap(), self.accum as u8);
            }
            OpCode::STX => {
                let (_, addr) = self.get_operand(inst.addr_mode);
                self.mem.write(addr.unwrap(), self.x as u8);
            }
            OpCode::STY => {
                let (_, addr) = self.get_operand(inst.addr_mode);
                self.mem.write(addr.unwrap(), self.y as u8);
            }
            OpCode::TAX => {
                self.x = self.accum;
            }
            OpCode::TAY => {
                self.y = self.accum;
            }
            OpCode::TSX => {
                self.x = self.sp as i8;
            }
            OpCode::TXA => {
                self.accum = self.x;
            }
            OpCode::TXS => {
                self.sp = self.x as u8;
            }
            OpCode::TYA => {
                self.accum = self.y;
            }
        }
        for _ in 0..inst.cycles {
            self.tick_clock();
        }
    }
    // Returns (operand, operand_addr)
    fn get_operand(&mut self, addr_mode: AddrMode) -> (i8, Option<u16>) {
        match addr_mode {
            AddrMode::Absolute => {
                let addr = self.mem.read(self.pc) as u16 | (self.mem.read(self.pc + 1) << 8) as u16;
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::AbsoluteX => {
                let addr = (self.mem.read(self.pc) as u16
                    | (self.mem.read(self.pc + 1) << 8) as u16)
                    + self.x as u16;
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::AbsoluteY => {
                let addr = (self.mem.read(self.pc) as u16
                    | (self.mem.read(self.pc + 1) << 8) as u16)
                    + self.y as u16;
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::Immediate => (self.mem.read(self.pc) as i8, None),
            AddrMode::ZeroPage => {
                let addr = mem::ZERO_PAGE_START + self.mem.read(self.pc) as u16;
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::ZeroPageX => {
                let addr =
                    mem::ZERO_PAGE_START + ((self.mem.read(self.pc) + self.x as u8) % 255) as u16;
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::ZeroPageY => {
                let addr =
                    mem::ZERO_PAGE_START + ((self.mem.read(self.pc) + self.y as u8) as u16 % 256);
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::Relative => (self.mem.read(self.pc) as i8, None),
            AddrMode::Indirect => {
                let in_addr =
                    self.mem.read(self.pc) as u16 | (self.mem.read(self.pc + 1) << 8) as u16;
                // Original 6502 doesn't fetch Indirect addresses correctly when the indirect address vector falls on a page boundary.
                // The logic below encodes this behavior.
                let addr = if (in_addr + 1) % 256 == 0 {
                    self.mem.read(in_addr) as u16 | (self.mem.read(in_addr + 1) << 8) as u16
                } else {
                    self.mem.read(in_addr) as u16 | (self.mem.read(in_addr & 0xFF00) << 8) as u16
                };
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::IndexedIndirect => {
                let in_addr =
                    mem::ZERO_PAGE_START + ((self.mem.read(self.pc) + self.x as u8) as u16 % 256);
                let addr = self.mem.read(in_addr) as u16 | (self.mem.read(in_addr + 1) << 8) as u16;
                (self.mem.read(addr) as i8, Some(addr))
            }
            AddrMode::IndirectIndexed => {
                let in_addr = mem::ZERO_PAGE_START + self.mem.read(self.pc) as u16;
                let addr = self.mem.read(in_addr) as u16 | (self.mem.read(in_addr + 1) << 8) as u16;
                (
                    self.mem.read(addr + self.y as u16) as i8,
                    Some(addr + self.y as u16),
                )
            }
            AddrMode::Accumulator => (self.accum as i8, None),
            AddrMode::Implicit => (0, None), // should never be used
        }
    }

    pub fn advance_cpu(&mut self) {
        let inst = self.fetch_instruction();
        self.execute_instruction(&inst);
        self.pc += (inst.size - 1) as u16;
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CPU")
            .field("pc", &self.pc)
            .field("sp", &self.sp)
            .field("accumulator", &self.accum)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("flags", &self.status)
            .field("cycle", &self.cycle)
            .finish()
    }
}

impl StatusRegister {
    pub fn new() -> StatusRegister {
        StatusRegister {
            c: 0,
            z: 0,
            i: 1, // Interrupts disabled at boot
            d: 0,
            b: 1,
            bit_5: 1,
            v: 0,
            n: 0,
        }
    }
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
    pub fn set_flags(&mut self, flags: u8) {
        self.c = flags & 1;
        self.z = flags & (1 << 1);
        self.i = flags & (1 << 2);
        self.d = flags & (1 << 3);
        self.b = flags & (1 << 4);
        self.bit_5 = flags & (1 << 5);
        self.v = flags & (1 << 6);
        self.n = flags & (1 << 7);
    }
}

// Checks if adding these numbers will result in an incorrect sign bit
fn signed_overflow_add(x: i8, y: i8) -> bool {
    (x > 0 && y > 0 && x.wrapping_add(y) <= 0) || (x < 0 && y < 0 && x.wrapping_add(y) >= 0)
}
// Checks if subtracting these numbers will result in an incorrect sign bit
fn signed_overflow_sub(x: i8, y: i8) -> bool {
    (x > 0 && y > 0 && x > y && x.wrapping_sub(y) <= 0)
        || (x > 0 && y > 0 && x < y && x.wrapping_sub(y) >= 0)
        || (x > 0 && y < 0 && x.wrapping_sub(y) < 0)
        || (x < 0 && y > 0 && x.wrapping_sub(y) > 0)
}

// checks if a page boundary is crossed
fn page_crossed(old_addr: u16, new_addr: u16) -> bool {
    old_addr >> 7 == new_addr >> 7
}
