use crate::cpu::CPU;
use crate::mem::Memory;
use crate::ppu::PPU;
use std::cell::RefCell;
use std::rc::Rc;

struct NES {
    cpu: CPU,
    ppu: PPU,
}

impl NES {
    pub fn new() -> NES {
        let mem = Rc::new(RefCell::new(Memory::new()));
        NES {
            cpu: CPU::new(mem.clone()),
            ppu: PPU::new(mem.clone()),
        }
    }
}
