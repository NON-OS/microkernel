// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_power_trim` and `rtw8822c_set_power_trim` (rtw8822c.c:1024-1091):
//! the factory TX gain trims into each path's RF gain table, written only
//! when some trim was burned, and DPD off for every rate either way.

use super::super::bits::write32_mask;
use super::super::power::{DIS_DPD_ALL, REG_DIS_DPD};
use super::super::rf::{self, RFREG_MASK};
use super::UNBURNED;
use crate::efuse::facts::Trim;
use crate::regs::Mmio;

/// The gain-table entries 0x0 to 0xE and the trim index each takes.
pub const ENTRY_GAIN: [u8; 15] = [0, 1, 2, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6, 7, 7];

/// The gains per path: the three 2.4 GHz trims (A in the low nibble, B in
/// the high), then the five 5 GHz ones of the first `paths`; `None` when no
/// trim was burned.
pub fn power_gains(t: &Trim, paths: usize) -> Option<[[u8; 8]; 2]> {
    let mut gain = [[0u8; 8]; 2];
    let mut set = false;
    for (i, &pg) in t.tx_2g.iter().enumerate().filter(|(_, &pg)| pg != UNBURNED) {
        (gain[0][i], gain[1][i], set) = (pg & 0xF, pg >> 4, true);
    }
    for (path, tx) in t.tx_5g.iter().enumerate().take(paths) {
        for (i, &pg) in tx.iter().enumerate().filter(|(_, &pg)| pg != UNBURNED) {
            (gain[path][3 + i], set) = (pg & 0x1F, true);
        }
    }
    set.then_some(gain)
}

/// The gains into the RF gain table (0x33 the entry, 0x3F its gain, inside
/// 0xEE bit 19), then DPD off.
pub fn power<M: Mmio>(mmio: &M, t: &Trim, paths: usize) {
    if let Some(gain) = power_gains(t, paths) {
        for (path, g) in gain.iter().enumerate().take(paths) {
            rf::write(mmio, path, 0xEE, 1 << 19, 1);
            for (seq, &idx) in ENTRY_GAIN.iter().enumerate() {
                rf::write(mmio, path, 0x33, RFREG_MASK, seq as u32);
                rf::write(mmio, path, 0x3F, RFREG_MASK, u32::from(g[idx as usize]));
            }
            rf::write(mmio, path, 0xEE, 1 << 19, 0);
        }
    }
    write32_mask(mmio, REG_DIS_DPD, DIS_DPD_ALL, DIS_DPD_ALL);
}
