// NONOS Operating System (AGPL-3.0-or-later)

//! The board facts the PHY needs, taken out of the two maps (rtw88
//! `rtw8822c_read_efuse` and the trim reads of `rtw8822c_rf_init`). The
//! station address in the map (0x120) is never read.

use super::layout::*;

/// One RF path's 2.4 GHz power indexes (`struct rtw_2g_txpwr_idx`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PathPower {
    /// CCK base per channel group (1-2, 3-5, 6-8, 9-11, 12-13, 14).
    pub cck: [u8; 6],
    /// HT40 base per channel group; OFDM and HT20 are diffs from it.
    pub bw40: [u8; 5],
    /// `ht_1s_diff`: OFDM diff in the low nibble, HT20 in the high, signed.
    pub diff: u8,
}

/// The factory trims in the physical map's protected area (`PPG_*`); 0xFF is
/// a byte never burned, which rtw88 takes as nothing to trim.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Trim {
    pub tx_2g: [u8; 3],
    pub tx_5g: [[u8; 5]; 2],
    pub thermal: [u8; 2],
    pub pa_bias_2g: [u8; 2],
    pub pa_bias_5g: [u8; 2],
}

/// The facts out of the maps: the RF front end (the whole byte, as rtw88
/// takes it), the crystal, both paths' power and the trims. `None` for a
/// front end never burned.
pub fn facts(
    phys: &[u8; PHYS_SIZE],
    log: &[u8; LOG_SIZE],
) -> Option<(u8, u8, [PathPower; 2], Trim)> {
    let rfe = log[RFE_OPTION];
    if rfe == 0xFF {
        return None;
    }
    let power = |p: usize| {
        let at = TXPWR_IDX + p * TXPWR_STRIDE;
        PathPower {
            cck: core::array::from_fn(|i| log[at + i]),
            bw40: core::array::from_fn(|i| log[at + 6 + i]),
            diff: log[at + 11],
        }
    };
    let trim = Trim {
        tx_2g: PPG_2G_TXAB.map(|a| phys[a]),
        tx_5g: PPG_5G_TX.map(|path| path.map(|a| phys[a])),
        thermal: PPG_THERMAL.map(|a| phys[a]),
        pa_bias_2g: PPG_PABIAS_2G.map(|a| phys[a]),
        pa_bias_5g: PPG_PABIAS_5G.map(|a| phys[a]),
    };
    Some((rfe, log[XTAL_K] & 0x7F, [power(0), power(1)], trim))
}
