// Little endian, 64kb addressable memory.
// Memory is mapped as follows:
// 0x0000-0x07FF - 2KB internal RAM
// 0x0800-0x1FFF - Mirrors of 0x0000-0x07FF
// 0x2000-0x2007 - NES PPU Registers
// 0x2008-0x3FFF - Mirrors of 0x2000-0x2007 (repeats every 8 bytes)
// 0x4000-0x4017 - NES APU and I/O registers
// 0x4018-0x401F - APU and I/O functionality that's normally disabled
// 0x4020-0xFFFF - Cartridge space: PRG ROM, PRG RAM, and mapper registers

pub const ZERO_PAGE_START: u16 = 0x00;
pub const STACK_TOP: u16 = 0x100;

pub struct Memory {
    ram: Box<[u8; 2048]>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: Box::new([0xFFu8; 2048]),
        }
    }
    // 2kb on-board memory
    pub fn ram_read(&self, addr: u16) -> u8 {
        self.ram[(addr % 2048) as usize]
    }
    pub fn ram_write(&mut self, addr: u16, data: u8) {
        self.ram[(addr & 2048) as usize] = data;
    }
}
