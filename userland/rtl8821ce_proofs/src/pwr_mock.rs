// NONOS Operating System (AGPL-3.0-or-later)
//! A modeled 8821C register file for the power-switch proofs. It logs every
//! byte write, clears the self-clearing power bits in 0x05 the way the chip
//! does, and can hold one bit stuck or raise a bit when BIT_PFM_WOWL pulses.

use core::cell::RefCell;

use crate::regs::Mmio;

pub struct Chip {
    pub regs: RefCell<[u8; 0x1800]>,
    pub writes: RefCell<Vec<(usize, u8)>>,
    /// (offset, bit) that never clears, to model a poll that never settles.
    pub stuck: Option<(usize, u8)>,
    /// (offset, bit) raised when BIT_PFM_WOWL in 0x04 is pulsed off.
    pub on_wowl: Option<(usize, u8)>,
}

impl Chip {
    pub fn new(cr: u8) -> Self {
        let c = Chip { regs: RefCell::new([0; 0x1800]), writes: RefCell::new(Vec::new()), stuck: None, on_wowl: None };
        c.regs.borrow_mut()[0x100] = cr;
        c.regs.borrow_mut()[0x06] = 0x02; // power ready
        c
    }
    pub fn wrote(&self, off: usize) -> bool {
        self.writes.borrow().iter().any(|w| w.0 == off)
    }
}

impl Mmio for Chip {
    fn read8(&self, off: usize) -> u8 {
        let mut v = self.regs.borrow()[off];
        if off == 0x05 {
            v &= !0x03; // power-on and power-off requests self-clear
        }
        if let Some((o, b)) = self.stuck {
            if o == off { v |= b; }
        }
        v
    }
    fn write8(&self, off: usize, val: u8) {
        let prev = self.regs.borrow()[off];
        self.regs.borrow_mut()[off] = val;
        self.writes.borrow_mut().push((off, val));
        if off == 0x04 && prev & 0x08 != 0 && val & 0x08 == 0 {
            if let Some((o, b)) = self.on_wowl { self.regs.borrow_mut()[o] |= b; }
        }
    }
    fn read16(&self, off: usize) -> u16 {
        u16::from_le_bytes([self.read8(off), self.read8(off + 1)])
    }
    fn write16(&self, off: usize, val: u16) {
        for (i, b) in val.to_le_bytes().iter().enumerate() { self.write8(off + i, *b); }
    }
    fn read32(&self, off: usize) -> u32 {
        u32::from_le_bytes([self.read8(off), self.read8(off + 1), self.read8(off + 2), self.read8(off + 3)])
    }
    fn write32(&self, off: usize, val: u32) {
        for (i, b) in val.to_le_bytes().iter().enumerate() { self.write8(off + i, *b); }
    }
}
