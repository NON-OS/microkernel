// NONOS Operating System (AGPL-3.0-or-later)

//! The card's register window, mapped from its memory BAR. An access outside
//! the window, or misaligned, reads all ones and writes nothing, so a bad
//! offset is a dead read the bring-up names, never a stray store.

use super::mmio::Mmio;

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

    /// The register of `n` bytes at `off`, if inside the window and aligned.
    fn at(&self, off: u32, n: usize) -> Option<usize> {
        let off = off as usize;
        let end = off.checked_add(n)?;
        (end <= self.len && off.is_multiple_of(n)).then(|| self.base + off)
    }
}

// SAFETY (every access below): `new`'s caller vouched for `len` bytes of
// mapped registers at `base`, and `at` put the access, aligned, inside them.
impl Mmio for Regs {
    fn read8(&self, off: u32) -> u8 {
        let Some(p) = self.at(off, 1) else { return u8::MAX };
        unsafe { nonos_libc::mmio_read8(p) }
    }
    fn write8(&self, off: u32, val: u8) {
        let Some(p) = self.at(off, 1) else { return };
        unsafe { nonos_libc::mmio_write8(p, val) }
    }
    fn read16(&self, off: u32) -> u16 {
        let Some(p) = self.at(off, 2) else { return u16::MAX };
        unsafe { nonos_libc::mmio_read16(p) }
    }
    fn write16(&self, off: u32, val: u16) {
        let Some(p) = self.at(off, 2) else { return };
        unsafe { nonos_libc::mmio_write16(p, val) }
    }
    fn read32(&self, off: u32) -> u32 {
        let Some(p) = self.at(off, 4) else { return u32::MAX };
        unsafe { nonos_libc::mmio_read32(p) }
    }
    fn write32(&self, off: u32, val: u32) {
        let Some(p) = self.at(off, 4) else { return };
        unsafe { nonos_libc::mmio_write32(p, val) }
    }
}
