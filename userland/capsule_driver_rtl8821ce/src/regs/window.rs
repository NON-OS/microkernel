// NONOS Operating System (AGPL-3.0-or-later)

//! The register window as the driver maps it.

use super::mmio::Mmio;

#[derive(Clone, Copy)]
pub struct Regs {
    base: usize,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base: base as usize }
    }
}

impl Mmio for Regs {
    fn read8(&self, off: usize) -> u8 {
        unsafe { core::ptr::read_volatile((self.base + off) as *const u8) }
    }
    fn write8(&self, off: usize, val: u8) {
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u8, val) }
    }
    fn read16(&self, off: usize) -> u16 {
        unsafe { core::ptr::read_volatile((self.base + off) as *const u16) }
    }
    fn write16(&self, off: usize, val: u16) {
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u16, val) }
    }
    fn read32(&self, off: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.base + off) as *const u32) }
    }
    fn write32(&self, off: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u32, val) }
    }
}
