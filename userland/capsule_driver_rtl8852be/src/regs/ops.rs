// NONOS Operating System (AGPL-3.0-or-later)

//! rtw89's read-modify-write helpers (`util.h` and `core.h` in Linux v6.12
//! `drivers/net/wireless/realtek/rtw89/`: `rtw89_write32_set`, `_clr`,
//! `_mask`, `rtw89_read32_mask`), over any `Mmio`. A mask write shifts the
//! value to the mask's lowest set bit and keeps every bit outside it.

use super::mmio::Mmio;

/// The value `data` placed under `mask`, as `u32_encode_bits` places it.
pub const fn field(mask: u32, data: u32) -> u32 {
    if mask == 0 {
        return 0;
    }
    (data << mask.trailing_zeros()) & mask
}

/// The field under `mask` in `val`, shifted down.
pub const fn get(val: u32, mask: u32) -> u32 {
    if mask == 0 {
        return 0;
    }
    (val & mask) >> mask.trailing_zeros()
}

pub trait Rw: Mmio {
    fn set32(&self, off: u32, bits: u32) {
        self.write32(off, self.read32(off) | bits);
    }
    fn clr32(&self, off: u32, bits: u32) {
        self.write32(off, self.read32(off) & !bits);
    }
    fn mask32(&self, off: u32, mask: u32, data: u32) {
        self.write32(off, (self.read32(off) & !mask) | field(mask, data));
    }
    fn get32(&self, off: u32, mask: u32) -> u32 {
        get(self.read32(off), mask)
    }
    fn set16(&self, off: u32, bits: u16) {
        self.write16(off, self.read16(off) | bits);
    }
    fn clr16(&self, off: u32, bits: u16) {
        self.write16(off, self.read16(off) & !bits);
    }
    fn mask16(&self, off: u32, mask: u16, data: u16) {
        let v = (self.read16(off) & !mask) | field(mask as u32, data as u32) as u16;
        self.write16(off, v);
    }
    fn set8(&self, off: u32, bits: u8) {
        self.write8(off, self.read8(off) | bits);
    }
    fn clr8(&self, off: u32, bits: u8) {
        self.write8(off, self.read8(off) & !bits);
    }
    fn mask8(&self, off: u32, mask: u8, data: u8) {
        let v = (self.read8(off) & !mask) | field(mask as u32, data as u32) as u8;
        self.write8(off, v);
    }
}

impl<T: Mmio + ?Sized> Rw for T {}
