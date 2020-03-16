pub struct Instruction {
    op: OpCode,
    addr_mode: AddrMode,
    cycles: u8, // cycles this instruction will take to execute
    size: u8, // size in bytes, so CPU knows how much to incrememnt PC and whether it needs to fetch more data from memory
}

pub enum AddrMode {
    Implicit,        // No further action necessary
    Accumulator,     // Operate directony on the accumulator
    Immediate,       // Operate on 1 byte constant specified in the instruction
    ZeroPage, // 8 bit address operand added to start of zero page (0x0000). So, the addressability of these instructions is 0x0000-0x00FF
    ZeroPageX, // address = immediate value + X register
    ZeroPageY, // address = immediate value + Y register
    Relative, // branch instructions. PC + signed immediate value
    Absolute, // Instruction contains 16 address bits
    AbsoluteX, // 16 bit immediate value + X register
    AbsoluteY, // 16 bit immediate value + Y register
    Indirect, // Instruction contains 16 bit address which points to the least significant byte of the real target address
    IndexedIndirect, // Instruction contains address of table in zero page. Address is added to X register with zero page wrap-around to get target address
    IndirectIndexed, // Instruction contains zero page address of least significant byte of a 16 bit address. This is added to the Y register to get the target address
}

pub enum OpCode {
    ADC, // Add With Carry
    AND, // Logical AND
    ASL, // Arithmetic Shift Left
    BCC, // Branch if Carry Clear
    BCS, // Branch if Carry Set
    BEQ, // Branch If Equal
    BIT, // Bit Test
    BMI, // Branch if Minus
    BNE, // Branch Not Equal
    BPL, // Branch if Positive
    BRK, // Force Interrupt
    BVC, // Branch if Overflow Clear
    BVS, // Branch if Overflow Set
    CLC, // Clear Carry Flag
    CLD, // Clear Decimal Mode
    CLI, // CLear Interrupt Disable
    CLV, // Clear Overflow Flag
    CMP, // Compare
    CPX, // Compare X
    CPY, // Compary Y
    DEC, // Decrement Memory
    DEX, // Decrement X
    DEY, // Decrement Y
    EOR, // XOR
    INC, // Incrememt Memory
    INX, // Increment X
    INY, // Increment Y
    JMP, // Jump
    JSR, // Jump to Subroutine
    LDA, // Load Accumulator
    LDX, // Load X
    LDY, // Load Y
    LSR, // Logical Shift Right
    NOP, // No-Op
    ORA, // Logical Inclusive OR
    PHA, // Push Accumulator
    PHP, // Push Processor Status
    PLA, // Pull Accumulator
    PLP, // Pull Processor Status
    ROL, // Rotate Left
    ROR, // Rotate Right
    RTI, // Return from Interrupt
    RTS, // Return from Subroutine
    SBC, // Subtract with Carry
    SEC, // Set Carry Flag
    SED, // Set Decimal Mode
    SEI, // Set Interrupt Disable
    STA, // Store Accumulator
    STX, // Store X
    STY, // Store Y
    TAX, // Transfer Accumulator to X
    TAY, // Transfer Accumulator to Y
    TSX, // Transfer Stack Pointer to X
    TXA, // Transfer X to Accumulator
    TXS, // Transfer X to Stack Pointer
    TYA, // Transfer Y to Accumulator
}

impl Instruction {
    pub fn new(opcode: u8) -> Instruction {
        // FAT ASS MATCH INCOMING
        match opcode {
            0x00 => Instruction {
                op: OpCode::BRK,
                addr_mode: AddrMode::Implicit,
                cycles: 7,
                size: 1,
            },
            0x01 => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0x05 => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x06 => Instruction {
                op: OpCode::ASL,
                addr_mode: AddrMode::ZeroPage,
                cycles: 5,
                size: 2,
            },
            0x08 => Instruction {
                op: OpCode::PHP,
                addr_mode: AddrMode::Implicit,
                cycles: 3,
                size: 1,
            },
            0x09 => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0x0a => Instruction {
                op: OpCode::ASL,
                addr_mode: AddrMode::Accumulator,
                cycles: 2,
                size: 1,
            },
            0x0d => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x0e => Instruction {
                op: OpCode::ASL,
                addr_mode: AddrMode::Absolute,
                cycles: 6,
                size: 3,
            },
            0x10 => Instruction {
                op: OpCode::BPL,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0x11 => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 5,
                size: 2,
            },
            0x15 => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0x16 => Instruction {
                op: OpCode::ASL,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 6,
                size: 2,
            },
            0x18 => Instruction {
                op: OpCode::CLC,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x19 => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 2,
            },
            0x1d => Instruction {
                op: OpCode::ORA,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0x1e => Instruction {
                op: OpCode::ASL,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 7,
                size: 3,
            },
            0x20 => Instruction {
                op: OpCode::JSR,
                addr_mode: AddrMode::Relative,
                cycles: 6,
                size: 3,
            },
            0x21 => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0x24 => Instruction {
                op: OpCode::BIT,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x25 => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x26 => Instruction {
                op: OpCode::ROL,
                addr_mode: AddrMode::ZeroPage,
                cycles: 5,
                size: 2,
            },
            0x28 => Instruction {
                op: OpCode::PLP,
                addr_mode: AddrMode::Implicit,
                cycles: 4,
                size: 1,
            },
            0x29 => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0x2a => Instruction {
                op: OpCode::ROL,
                addr_mode: AddrMode::Accumulator,
                cycles: 2,
                size: 1,
            },
            0x2c => Instruction {
                op: OpCode::BIT,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x2d => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x2e => Instruction {
                op: OpCode::ROL,
                addr_mode: AddrMode::Absolute,
                cycles: 6,
                size: 3,
            },
            0x30 => Instruction {
                op: OpCode::BMI,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0x31 => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 5,
                size: 2,
            },
            0x35 => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0x36 => Instruction {
                op: OpCode::ROL,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 6,
                size: 2,
            },
            0x38 => Instruction {
                op: OpCode::SEC,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x39 => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 3,
            },
            0x3d => Instruction {
                op: OpCode::AND,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0x3e => Instruction {
                op: OpCode::ROL,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 7,
                size: 3,
            },
            0x40 => Instruction {
                op: OpCode::RTI,
                addr_mode: AddrMode::Implicit,
                cycles: 6,
                size: 1,
            },
            0x41 => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0x45 => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x46 => Instruction {
                op: OpCode::LSR,
                addr_mode: AddrMode::ZeroPage,
                cycles: 5,
                size: 2,
            },
            0x48 => Instruction {
                op: OpCode::PHA,
                addr_mode: AddrMode::Implicit,
                cycles: 3,
                size: 1,
            },
            0x49 => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0x4a => Instruction {
                op: OpCode::LSR,
                addr_mode: AddrMode::Accumulator,
                cycles: 2,
                size: 1,
            },
            0x4c => Instruction {
                op: OpCode::JMP,
                addr_mode: AddrMode::Absolute,
                cycles: 3,
                size: 3,
            },
            0x4d => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x4e => Instruction {
                op: OpCode::LSR,
                addr_mode: AddrMode::Absolute,
                cycles: 6,
                size: 3,
            },
            0x50 => Instruction {
                op: OpCode::BVC,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0x51 => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 5,
                size: 2,
            },
            0x55 => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0x56 => Instruction {
                op: OpCode::LSR,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 6,
                size: 2,
            },
            0x58 => Instruction {
                op: OpCode::CLI,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x59 => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 3,
            },
            0x5d => Instruction {
                op: OpCode::EOR,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0x5e => Instruction {
                op: OpCode::LSR,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 7,
                size: 3,
            },
            0x60 => Instruction {
                op: OpCode::RTS,
                addr_mode: AddrMode::Implicit,
                cycles: 6,
                size: 1,
            },
            0x61 => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0x65 => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x66 => Instruction {
                op: OpCode::ROR,
                addr_mode: AddrMode::ZeroPage,
                cycles: 5,
                size: 2,
            },
            0x68 => Instruction {
                op: OpCode::PLA,
                addr_mode: AddrMode::Implicit,
                cycles: 4,
                size: 1,
            },
            0x69 => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0x6a => Instruction {
                op: OpCode::ROR,
                addr_mode: AddrMode::Accumulator,
                cycles: 2,
                size: 1,
            },
            0x6c => Instruction {
                op: OpCode::JMP,
                addr_mode: AddrMode::Indirect,
                cycles: 5,
                size: 3,
            },
            0x6d => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x6e => Instruction {
                op: OpCode::ROR,
                addr_mode: AddrMode::Absolute,
                cycles: 6,
                size: 3,
            },
            0x70 => Instruction {
                op: OpCode::BVS,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0x71 => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 5,
                size: 2,
            },
            0x75 => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0x76 => Instruction {
                op: OpCode::ROR,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 6,
                size: 2,
            },
            0x78 => Instruction {
                op: OpCode::SEI,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x79 => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 3,
            },
            0x7d => Instruction {
                op: OpCode::ADC,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0x7e => Instruction {
                op: OpCode::ROR,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 7,
                size: 3,
            },
            0x81 => Instruction {
                op: OpCode::STA,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0x84 => Instruction {
                op: OpCode::STY,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x85 => Instruction {
                op: OpCode::STA,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x86 => Instruction {
                op: OpCode::STX,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0x88 => Instruction {
                op: OpCode::DEY,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x8a => Instruction {
                op: OpCode::TXA,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x8c => Instruction {
                op: OpCode::STY,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x8d => Instruction {
                op: OpCode::STA,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x8e => Instruction {
                op: OpCode::STX,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0x90 => Instruction {
                op: OpCode::BCC,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0x91 => Instruction {
                op: OpCode::STA,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 6,
                size: 2,
            },
            0x94 => Instruction {
                op: OpCode::STY,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0x95 => Instruction {
                op: OpCode::STA,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0x96 => Instruction {
                op: OpCode::STX,
                addr_mode: AddrMode::ZeroPageY,
                cycles: 4,
                size: 2,
            },
            0x98 => Instruction {
                op: OpCode::TYA,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x99 => Instruction {
                op: OpCode::STA,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 5,
                size: 3,
            },
            0x9a => Instruction {
                op: OpCode::TXS,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0x9d => Instruction {
                op: OpCode::STA,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 5,
                size: 3,
            },
            0xa0 => Instruction {
                op: OpCode::LDY,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0xa1 => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0xa2 => Instruction {
                op: OpCode::LDX,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0xa4 => Instruction {
                op: OpCode::LDY,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0xa5 => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0xa6 => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0xa8 => Instruction {
                op: OpCode::TAY,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xa9 => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0xaa => Instruction {
                op: OpCode::TAX,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xac => Instruction {
                op: OpCode::LDY,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0xad => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0xae => Instruction {
                op: OpCode::LDX,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0xb0 => Instruction {
                op: OpCode::BCS,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0xb1 => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 5,
                size: 2,
            },
            0xb4 => Instruction {
                op: OpCode::LDY,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0xb5 => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0xb6 => Instruction {
                op: OpCode::LDX,
                addr_mode: AddrMode::ZeroPageY,
                cycles: 4,
                size: 2,
            },
            0xb8 => Instruction {
                op: OpCode::CLV,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xb9 => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 3,
            },
            0xba => Instruction {
                op: OpCode::TSX,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xbc => Instruction {
                op: OpCode::LDY,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0xbd => Instruction {
                op: OpCode::LDA,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0xbe => Instruction {
                op: OpCode::LDX,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 3,
            },
            0xc0 => Instruction {
                op: OpCode::CPY,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0xc1 => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0xc4 => Instruction {
                op: OpCode::CPY,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0xc5 => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0xc6 => Instruction {
                op: OpCode::DEC,
                addr_mode: AddrMode::ZeroPage,
                cycles: 5,
                size: 2,
            },
            0xc8 => Instruction {
                op: OpCode::INY,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xc9 => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0xca => Instruction {
                op: OpCode::DEX,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xcc => Instruction {
                op: OpCode::CPY,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0xcd => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0xce => Instruction {
                op: OpCode::DEC,
                addr_mode: AddrMode::Absolute,
                cycles: 6,
                size: 3,
            },
            0xd0 => Instruction {
                op: OpCode::BNE,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0xd1 => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 5,
                size: 2,
            },
            0xd5 => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0xd6 => Instruction {
                op: OpCode::DEC,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 6,
                size: 2,
            },
            0xd8 => Instruction {
                op: OpCode::CLD,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xd9 => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 3,
            },
            0xdd => Instruction {
                op: OpCode::CMP,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0xde => Instruction {
                op: OpCode::DEC,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 7,
                size: 3,
            },
            0xe0 => Instruction {
                op: OpCode::CPX,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0xe1 => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::IndexedIndirect,
                cycles: 6,
                size: 2,
            },
            0xe4 => Instruction {
                op: OpCode::CPX,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0xe5 => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::ZeroPage,
                cycles: 3,
                size: 2,
            },
            0xe6 => Instruction {
                op: OpCode::INC,
                addr_mode: AddrMode::ZeroPage,
                cycles: 5,
                size: 2,
            },
            0xe8 => Instruction {
                op: OpCode::INX,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xe9 => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::Immediate,
                cycles: 2,
                size: 2,
            },
            0xea => Instruction {
                op: OpCode::NOP,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xec => Instruction {
                op: OpCode::CPX,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0xed => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::Absolute,
                cycles: 4,
                size: 3,
            },
            0xee => Instruction {
                op: OpCode::INC,
                addr_mode: AddrMode::Absolute,
                cycles: 6,
                size: 3,
            },
            0xf0 => Instruction {
                op: OpCode::BEQ,
                addr_mode: AddrMode::Relative,
                cycles: 2,
                size: 2,
            },
            0xf1 => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::IndirectIndexed,
                cycles: 5,
                size: 2,
            },
            0xf5 => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 4,
                size: 2,
            },
            0xf6 => Instruction {
                op: OpCode::INC,
                addr_mode: AddrMode::ZeroPageX,
                cycles: 6,
                size: 2,
            },
            0xf8 => Instruction {
                op: OpCode::SED,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
            0xf9 => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::AbsoluteY,
                cycles: 4,
                size: 3,
            },
            0xfd => Instruction {
                op: OpCode::SBC,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 4,
                size: 3,
            },
            0xfe => Instruction {
                op: OpCode::INC,
                addr_mode: AddrMode::AbsoluteX,
                cycles: 7,
                size: 3,
            },
            // TODO maybe fix this
            _ => Instruction {
                op: OpCode::NOP,
                addr_mode: AddrMode::Implicit,
                cycles: 2,
                size: 1,
            },
        }
    }
}
