// NONOS Operating System (AGPL-3.0-or-later)

//! The tail of rtw88 `rtw8822c_rf_init` (rtw8822c.c:1010-1131) after the
//! DAC calibration: the crystal doubler check, then the factory thermal,
//! power (`power`) and PA-bias trims from the efuse's protected area, each
//! written to the RF registers as rtw88 writes them, and skipped as rtw88
//! skips it.

pub mod power;

use super::rf::{self, PATH_A, RFREG_MASK};
use crate::efuse::facts::Trim;
use crate::regs::Mmio;
use crate::wait::delay_ms;

pub use power::power;

/// What an efuse byte reads that was never burned (`EFUSE_READ_FAIL`).
pub const UNBURNED: u8 = 0xFF;

/// `rtw8822c_rf_x2_check`: a crystal doubler still busy is restarted.
pub fn x2_check<M: Mmio>(mmio: &M) {
    delay_ms(1);
    if rf::read(mmio, PATH_A, 0xB8, 1 << 15) == 1 {
        rf::write(mmio, PATH_A, 0xB8, RFREG_MASK, 0xC4440);
        rf::write(mmio, PATH_A, 0xBA, RFREG_MASK, 0x6840D);
        rf::write(mmio, PATH_A, 0xB8, RFREG_MASK, 0x80440);
        delay_ms(1);
    }
}

/// The thermal trim's RF value: efuse bit 0 moved to bit 3, bits 3:1 down
/// one (rtw88's own comment).
pub fn thermal_value(pg: u8) -> u8 {
    ((pg >> 1) & 0x7) | ((pg & 1) << 3)
}

/// `rtw8822c_thermal_trim`: stops at the first path never trimmed.
pub fn thermal<M: Mmio>(mmio: &M, t: &Trim, paths: usize) {
    for (path, &pg) in t.thermal.iter().enumerate().take(paths) {
        if pg == UNBURNED {
            return;
        }
        rf::write(mmio, path, 0x43, 0xF_0000, u32::from(thermal_value(pg))); // RF_THEMAL_MASK
    }
}

/// `rtw8822c_pa_bias`: the 2.4 GHz bias per path, stopping at the first never
/// burned (which skips the 5 GHz one too), then the 5 GHz one, which rtw88
/// writes unchecked.
pub fn pa_bias<M: Mmio>(mmio: &M, t: &Trim, paths: usize) {
    for (path, &pg) in t.pa_bias_2g.iter().enumerate().take(paths) {
        if pg == UNBURNED {
            return;
        }
        rf::write(mmio, path, 0x60, 0xF000, u32::from(pg & 0xF)); // RF_PA, 2G mask
    }
    for (path, &pg) in t.pa_bias_5g.iter().enumerate().take(paths) {
        rf::write(mmio, path, 0x60, 0xF_0000, u32::from(pg & 0xF)); // 5G mask
    }
}
