// NONOS Operating System (AGPL-3.0-or-later)
//! The modelled chip as the driver's register access sees it.

use super::Chip;
use crate::regs::Mmio;

impl Mmio for Chip {
    fn read8(&self, off: usize) -> u8 {
        self.get(off, 1) as u8
    }
    fn write8(&self, off: usize, v: u8) {
        self.put(off, 1, u32::from(v))
    }
    fn read16(&self, off: usize) -> u16 {
        self.get(off, 2) as u16
    }
    fn write16(&self, off: usize, v: u16) {
        self.put(off, 2, u32::from(v))
    }
    fn read32(&self, off: usize) -> u32 {
        self.get(off, 4)
    }
    fn write32(&self, off: usize, v: u32) {
        self.put(off, 4, v)
    }
}
