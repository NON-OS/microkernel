// NONOS Operating System (AGPL-3.0-or-later)

//! rtw88's register field helpers (hci.h), as the 8822C PHY code uses them:
//! a masked write always reads the register first, keeps what is outside the
//! mask and shifts the value up to the mask's lowest bit.

use crate::regs::Mmio;

/// `rtw_write32_mask`. A zero mask writes the register back unchanged.
pub fn write32_mask<M: Mmio>(mmio: &M, addr: usize, mask: u32, data: u32) {
    let shift = mask.trailing_zeros() % 32;
    let orig = mmio.read32(addr);
    mmio.write32(addr, (orig & !mask) | ((data << shift) & mask));
}

/// `rtw_read32_mask`: the field under `mask`, shifted down.
pub fn read32_mask<M: Mmio>(mmio: &M, addr: usize, mask: u32) -> u32 {
    (mmio.read32(addr) & mask) >> (mask.trailing_zeros() % 32)
}

/// `rtw_write8_mask`.
pub fn write8_mask<M: Mmio>(mmio: &M, addr: usize, mask: u8, data: u8) {
    let shift = mask.trailing_zeros() % 8;
    let orig = mmio.read8(addr);
    mmio.write8(addr, (orig & !mask) | ((data << shift) & mask));
}

/// `rtw_write32_set` and `rtw_write32_clr`.
pub fn set32<M: Mmio>(mmio: &M, addr: usize, bits: u32) {
    let v = mmio.read32(addr);
    mmio.write32(addr, v | bits);
}

pub fn clr32<M: Mmio>(mmio: &M, addr: usize, bits: u32) {
    let v = mmio.read32(addr);
    mmio.write32(addr, v & !bits);
}

/// `rtw_write8_set` and `rtw_write8_clr`.
pub fn set8<M: Mmio>(mmio: &M, addr: usize, bits: u8) {
    let v = mmio.read8(addr);
    mmio.write8(addr, v | bits);
}

pub fn clr8<M: Mmio>(mmio: &M, addr: usize, bits: u8) {
    let v = mmio.read8(addr);
    mmio.write8(addr, v & !bits);
}

/// rtw88 `check_hw_ready`: the field under `mask` reads `target` within a
/// millisecond.
pub fn ready<M: Mmio>(mmio: &M, addr: usize, mask: u32, target: u32) -> bool {
    crate::wait::until_ms(1, || read32_mask(mmio, addr, mask) == target)
}
