// Little endian, 64kb addressable memory.
// Memory is mapped as follows:
// 0x0000-0x07FF - 2KB internal RAM
// 0x0800-0x1FFF - Mirrors of 0x0000-0x07FF
// 0x2000-0x2007 - NES PPU Registers
// 0x2008-0x3FFF - Mirrors of 0x2000-0x2007 (repeats every 8 bytes)
// 0x4000-0x4017 - NES APU and I/O registers
// 0x4018-0x401F - APU and I/O functionality that's normally disabled
// 0x4020-0xFFFF - Cartridge space: PRG ROM, PRG RAM, and mapper registers
pub struct Memory {
    // TODO maybe consider breaking this up into fields? e.g. internal RAM, PPU regs, etc
    mem: Box<[u8; 65536]>,
}

impl Memory {}
