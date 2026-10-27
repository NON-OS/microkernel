// NONOS Operating System (AGPL-3.0-or-later)

//! Each table walked by the condition parser and its writes made: the
//! baseband and calibration-init tables as `rtw_phy_cfg_bb` (32-bit writes,
//! 0xF9..=0xFE settle delays), AGC as `rtw_phy_cfg_agc`, the RF tables as
//! `rtw_phy_cfg_rf` (0xFFE and 0xFE delays, else a whole-register RF write).
//! rtw88 loads `rf_tbl[0]`, the path-B table, first, and then `rf_tbl[1]`,
//! the path-A table, one per RF path the chip has (`rtw8822c_hw_spec`).

use super::super::bits::write32_mask;
use super::super::cond::{apply, PhyCond};
use super::super::rf::{self, PATH_A, PATH_B, RFREG_MASK};
use super::super::tables::{RTL8822C_AGC, RTL8822C_BB, RTL8822C_CAL_INIT};
use super::super::tables::{RTL8822C_RF_A, RTL8822C_RF_B};
use super::delay::{bb_delay_ms, rf_delay_ms, rf_write_settle};
use crate::regs::Mmio;
use crate::wait::delay_ms;

/// `chip->rf_tbl`, in its order: the table and the path its writes go to.
const RF_TABLES: [(&[u32], usize); 2] = [(RTL8822C_RF_B, PATH_B), (RTL8822C_RF_A, PATH_A)];

fn cfg_bb<M: Mmio>(mmio: &M, addr: u32, data: u32) {
    match bb_delay_ms(addr) {
        Some(ms) => delay_ms(ms),
        None => mmio.write32(addr as usize, data),
    }
}

/// `rtw_load_rfk_table`: the IQK/DPK clocks and paths on, then the table.
fn load_cal_init<M: Mmio>(mmio: &M, cond: &PhyCond) {
    write32_mask(mmio, 0x1E24, 1 << 17, 1);
    write32_mask(mmio, 0x1CD0, 1 << 28, 1);
    write32_mask(mmio, 0x1CD0, 1 << 29, 1);
    write32_mask(mmio, 0x1CD0, 1 << 30, 1);
    write32_mask(mmio, 0x1CD0, 1 << 31, 0);
    apply(RTL8822C_CAL_INIT, cond, |a, d| cfg_bb(mmio, a, d));
}

/// Load every table for the chip `cond` describes, its RF tables for the
/// first `paths` entries of `RF_TABLES`.
pub fn load_all<M: Mmio>(mmio: &M, cond: &PhyCond, paths: usize) {
    apply(RTL8822C_BB, cond, |a, d| cfg_bb(mmio, a, d));
    apply(RTL8822C_AGC, cond, |a, d| mmio.write32(a as usize, d));
    load_cal_init(mmio, cond);
    for (table, path) in RF_TABLES.iter().take(paths) {
        apply(table, cond, |addr, data| {
            if let Some(ms) = rf_delay_ms(addr) {
                delay_ms(ms);
            } else {
                rf::write(mmio, *path, addr as u8, RFREG_MASK, data);
                rf_write_settle();
            }
        });
    }
}
