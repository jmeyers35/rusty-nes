use crate::mem::Memory;
use std::cell::RefCell;
use std::rc::Rc;
pub struct PPU {
    mem: Rc<RefCell<Memory>>,
}

impl PPU {
    pub fn new(mem: Rc<RefCell<Memory>>) -> PPU {
        PPU { mem }
    }
}
