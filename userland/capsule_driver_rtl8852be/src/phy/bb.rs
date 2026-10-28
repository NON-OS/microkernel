// NONOS Operating System (AGPL-3.0-or-later)

//! Baseband register access: rtw89's `rtw89_phy_write32*` and
//! `rtw89_phy_read32*` (Linux v6.12 rtw89 `phy.h`) add the AX PHY's
//! `cr_base`, 0x10000 (`rtw89_phy_gen_ax`, `phy.c`), to every offset. The
//! 8852B runs one PHY (no DBCC), so the `_idx` forms are the plain ones.

use crate::regs::{Mmio, Rw};

pub const CR_BASE: u32 = 0x1_0000;

/// A baseband view over the MAC's register window.
pub struct Bb<'a, R: Mmio>(pub &'a R);

impl<R: Mmio> Bb<'_, R> {
    pub fn write32(&self, addr: u32, v: u32) {
        self.0.write32(addr + CR_BASE, v);
    }
    pub fn read32(&self, addr: u32) -> u32 {
        self.0.read32(addr + CR_BASE)
    }
    pub fn mask(&self, addr: u32, mask: u32, v: u32) {
        self.0.mask32(addr + CR_BASE, mask, v);
    }
    pub fn get(&self, addr: u32, mask: u32) -> u32 {
        self.0.get32(addr + CR_BASE, mask)
    }
    pub fn set(&self, addr: u32, bits: u32) {
        self.0.set32(addr + CR_BASE, bits);
    }
    pub fn clr(&self, addr: u32, bits: u32) {
        self.0.clr32(addr + CR_BASE, bits);
    }
}
