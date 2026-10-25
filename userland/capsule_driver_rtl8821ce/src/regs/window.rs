// NONOS Operating System (AGPL-3.0-or-later)

//! The register window as the driver maps it. Every access is checked against
//! the mapping: an offset past it, or one not aligned to the access, reads all
//! ones, as a bus with nothing behind it does, and a write there goes nowhere.

use super::mmio::Mmio;

#[derive(Clone, Copy)]
pub struct Regs {
    base: usize,
    len: usize,
}

impl Regs {
    /// # Safety
    /// `base` must be the start of `len` bytes of register window mapped for
    /// this process (as `setup::run` maps the BAR) for as long as this lives.
    pub const unsafe fn new(base: u64, len: u64) -> Self {
        Self { base: base as usize, len: len as usize }
    }

    /// The register of `n` bytes at `off`, if it lies inside the window and is
    /// aligned to `n`.
    fn at(&self, off: usize, n: usize) -> Option<usize> {
        let end = off.checked_add(n)?;
        (end <= self.len && off.is_multiple_of(n)).then(|| self.base + off)
    }
}

// SAFETY (every access below): `new`'s caller vouched for `len` bytes of
// mapped registers at `base`, and `at` put the access, aligned, inside them.
impl Mmio for Regs {
    fn read8(&self, off: usize) -> u8 {
        let Some(p) = self.at(off, 1) else { return u8::MAX };
        unsafe { nonos_libc::mmio_read8(p) }
    }
    fn write8(&self, off: usize, val: u8) {
        let Some(p) = self.at(off, 1) else { return };
        unsafe { nonos_libc::mmio_write8(p, val) }
    }
    fn read16(&self, off: usize) -> u16 {
        let Some(p) = self.at(off, 2) else { return u16::MAX };
        unsafe { nonos_libc::mmio_read16(p) }
    }
    fn write16(&self, off: usize, val: u16) {
        let Some(p) = self.at(off, 2) else { return };
        unsafe { nonos_libc::mmio_write16(p, val) }
    }
    fn read32(&self, off: usize) -> u32 {
        let Some(p) = self.at(off, 4) else { return u32::MAX };
        unsafe { nonos_libc::mmio_read32(p) }
    }
    fn write32(&self, off: usize, val: u32) {
        let Some(p) = self.at(off, 4) else { return };
        unsafe { nonos_libc::mmio_write32(p, val) }
    }
}
