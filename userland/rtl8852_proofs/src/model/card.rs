// NONOS Operating System (AGPL-3.0-or-later)
//! The register window as the driver holds it. Every access must be aligned
//! to its width and inside the BAR the driver demands (`discover::BAR_MIN`):
//! the shipping window refuses anything else, so the model counts it as a
//! violation. Narrow accesses are carried out on the 32-bit word.

use super::state::State;
use super::{on_read, on_write};
use crate::discover::BAR_MIN;
use crate::regs::Mmio;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Card(pub Rc<RefCell<State>>);

impl Card {
    pub fn new() -> Self {
        Card(Rc::new(RefCell::new(State::new())))
    }

    fn checked(&self, off: u32, n: u32) -> bool {
        let ok = off % n == 0 && (off + n) as u64 <= BAR_MIN;
        if !ok {
            self.0.borrow_mut().violations.push(format!("access {off:#x} width {n}"));
        }
        ok
    }

    fn read(&self, off: u32, n: u32) -> u32 {
        if !self.checked(off, n) {
            return u32::MAX;
        }
        let a = off & !3;
        let v = on_read::read(&mut self.0.borrow_mut(), a);
        ((v as u64 >> ((off & 3) * 8)) & ((1u64 << (n * 8)) - 1)) as u32
    }

    fn write(&self, off: u32, n: u32, val: u32) {
        if !self.checked(off, n) {
            return;
        }
        let (a, shift) = (off & !3, (off & 3) * 8);
        let mask = if n == 4 { u32::MAX } else { ((1u32 << (n * 8)) - 1) << shift };
        let mut s = self.0.borrow_mut();
        let old = s.word(a);
        let new = (old & !mask) | ((val << shift) & mask);
        s.regs.insert(a, new);
        s.log.push((a, new));
        on_write::wrote(&mut s, a, old, new, mask);
    }
}

impl Mmio for Card {
    fn read8(&self, off: u32) -> u8 {
        self.read(off, 1) as u8
    }
    fn write8(&self, off: u32, val: u8) {
        self.write(off, 1, val as u32)
    }
    fn read16(&self, off: u32) -> u16 {
        self.read(off, 2) as u16
    }
    fn write16(&self, off: u32, val: u16) {
        self.write(off, 2, val as u32)
    }
    fn read32(&self, off: u32) -> u32 {
        self.read(off, 4)
    }
    fn write32(&self, off: u32, val: u32) {
        self.write(off, 4, val)
    }
}
