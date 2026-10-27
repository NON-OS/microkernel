// NONOS Operating System (AGPL-3.0-or-later)

//! A path's power index for each rate family, from its efuse entry (rtw88
//! `rtw_phy_get_2g_tx_power_index`, phy.c): CCK from `cck_base`, OFDM from
//! `bw40_base` plus the OFDM diff, HT20 from `bw40_base` plus the HT20 diff,
//! each diff a signed half-dB nibble worth two quarter-dB steps. A base the
//! efuse never burned (0xFF) gives the cap, as rtw88 clamps it there.

use nonos_wifi_core::regdom::max_eirp_dbm;

use crate::efuse::facts::PathPower;

/// The antenna gain assumed between conducted power and EIRP (2 dBi).
pub const ANTENNA_GAIN_DBI: u8 = 2;
/// `max_power_index` of the 8822C.
pub const MAX_POWER_INDEX: u8 = 0x7F;
/// Index steps per dB (`txgi_factor` 2 on half-dB units).
const STEPS_PER_DB: u8 = 4;
/// The channel groups of channels 1 to 13 (`rtw_get_channel_group`), the
/// channels the scan and the join tune to (`SCAN_CHANNELS`); channel 14's
/// CCK group 5 is never used.
const GROUPS: core::ops::Range<usize> = 0..5;

/// One path's indexes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Indexes {
    pub cck: u8,
    pub ofdm: u8,
    pub ht: u8,
}

/// The cap: the world domain's lowest 2.4 GHz EIRP less the antenna gain, in
/// index steps, and never above `MAX_POWER_INDEX`.
pub fn cap() -> u8 {
    let per_channel =
        |ch| max_eirp_dbm(ch)?.checked_sub(ANTENNA_GAIN_DBI)?.checked_mul(STEPS_PER_DB);
    (1..=14).filter_map(per_channel).min().unwrap_or(0).min(MAX_POWER_INDEX)
}

/// A signed 4-bit diff, in index steps.
fn diff(nibble: u8) -> i16 {
    i16::from(((nibble << 4) as i8) >> 4) * 2
}

fn capped(base: u8, delta: i16, cap: u8) -> u8 {
    if base == 0xFF {
        return cap;
    }
    (i16::from(base) + delta).clamp(0, i16::from(cap)) as u8
}

/// `p`'s indexes, the lowest over the channel groups, each at most `cap`.
pub fn indexes(p: &PathPower, cap: u8) -> Indexes {
    Indexes {
        cck: low(|g| capped(p.cck[g], 0, cap)),
        ofdm: low(|g| capped(p.bw40[g], diff(p.diff), cap)),
        ht: low(|g| capped(p.bw40[g], diff(p.diff >> 4), cap)),
    }
}

fn low(f: impl Fn(usize) -> u8) -> u8 {
    GROUPS.map(f).min().unwrap_or(0)
}
