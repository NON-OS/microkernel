// NONOS Operating System (AGPL-3.0-or-later)

//! What the DAC calibration changes, saved before and put back after
//! (rtw88 `rtw8822c_dac_backup_reg` / `_restore_reg`, rtw8822c.c:108-154),
//! and the baseband set up for it (`rtw8822c_dac_bb_setting`).
//!
//! rtw88 keeps RF 0x8F of both paths in one slot (`backup_rf[path * i + i]`
//! with i always 0), so path B's value is what both paths get back. That is
//! what every RTL8822CE under Linux runs, and this does the same.

use crate::phy::bits::write32_mask;
use crate::phy::rf::{self, PATH_A, PATH_B, RFREG_MASK};
use crate::regs::Mmio;

/// `DACK_REG_8822C`: the baseband registers saved.
pub const SAVED: [usize; 16] = [
    0x180C, 0x1810, 0x410C, 0x4110, 0x1C3C, 0x1C24, 0x1D70, 0x09B4, 0x1A00, 0x1A14, 0x1D58, 0x1C38,
    0x1E24, 0x1E28, 0x1860, 0x4160,
];
/// The RF register saved (`DACK_RF_8822C`).
const RF_SAVED: u8 = 0x8F;

pub struct Saved {
    bb: [u32; 16],
    rf_8f: u32,
}

pub fn save<M: Mmio>(mmio: &M) -> Saved {
    let bb = SAVED.map(|r| mmio.read32(r));
    // Both paths are read into the one slot; path B's read is the one kept.
    let _ = rf::read(mmio, PATH_A, RF_SAVED, RFREG_MASK);
    let rf_8f = rf::read(mmio, PATH_B, RF_SAVED, RFREG_MASK);
    Saved { bb, rf_8f }
}

pub fn restore<M: Mmio>(mmio: &M, s: &Saved) {
    for (r, v) in SAVED.iter().zip(s.bb) {
        mmio.write32(*r, v);
    }
    for path in [PATH_A, PATH_B] {
        rf::write(mmio, path, RF_SAVED, RFREG_MASK, s.rf_8f);
    }
}

pub fn bb_setting<M: Mmio>(mmio: &M) {
    write32_mask(mmio, 0x1D58, 0xFF8, 0x1FF);
    write32_mask(mmio, 0x1A00, 0x3, 0x2);
    write32_mask(mmio, 0x1A14, 0x300, 0x3);
    mmio.write32(0x1D70, 0x7E7E_7E7E);
    write32_mask(mmio, 0x180C, 0x3, 0x0);
    write32_mask(mmio, 0x410C, 0x3, 0x0);
    mmio.write32(0x1B00, 0x0000_0008);
    mmio.write8(0x1BCC, 0x3F);
    mmio.write32(0x1B00, 0x0000_000A);
    mmio.write8(0x1BCC, 0x3F);
    write32_mask(mmio, 0x1E24, 1 << 31, 0x0);
    write32_mask(mmio, 0x1E28, 0xF, 0x3);
}
