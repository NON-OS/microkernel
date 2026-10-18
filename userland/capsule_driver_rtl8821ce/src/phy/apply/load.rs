// NONOS Operating System (AGPL-3.0-or-later)

//! The PHY tables loaded, each under the board's conditions.

use super::super::cond::{apply, PhyCond};
use super::super::rf;
use super::super::tables::{RTL8821C_AGC, RTL8821C_BB, RTL8821C_MAC, RTL8821C_RF_A};
use super::delay::{bb_delay_ms, rf_delay_ms, rf_write_settle};
use crate::regs::Mmio;
use crate::wait::delay_ms;

/// Load the PHY-side MAC table (byte writes).
pub fn load_mac<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_MAC, drv, |addr, data| mmio.write8(addr as usize, data as u8));
}

/// Load the baseband table (32-bit writes, with inline settle delays).
pub fn load_bb<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_BB, drv, |addr, data| match bb_delay_ms(addr) {
        Some(ms) => delay_ms(ms),
        None => mmio.write32(addr as usize, data),
    });
}

/// Load the AGC table (32-bit writes).
pub fn load_agc<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_AGC, drv, |addr, data| mmio.write32(addr as usize, data));
}

/// Load the RF path-A table through the SIPI interface (with settle delays).
pub fn load_rf_a<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_RF_A, drv, |addr, data| {
        if let Some(ms) = rf_delay_ms(addr) {
            delay_ms(ms);
        } else {
            rf::write_a(mmio, addr as u8, data);
            rf_write_settle();
        }
    });
}

/// Load every PHY table in the rtw88 order: MAC, baseband, AGC, then RF.
pub fn load_all<M: Mmio>(mmio: &M, drv: &PhyCond) {
    load_mac(mmio, drv);
    load_bb(mmio, drv);
    load_agc(mmio, drv);
    load_rf_a(mmio, drv);
}
