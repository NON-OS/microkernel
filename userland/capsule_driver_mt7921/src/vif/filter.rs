/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The receive filter, `MCU_CE_CMD_SET_RX_FILTER` (68 bytes, Linux v6.12
//! `mt7921_mcu_set_rxfilter`): either the whole filter word (mode 1, as
//! `mt7921_configure_filter` sends it: enabled, no FCS-failed, control or
//! other-BSS frames) or one bit set or cleared (mode 2). The bit this
//! driver clears is `MT_WF_RFCR_DROP_OTHER_BEACON`, as Linux clears it
//! when no association filters beacons (`mt7921_mcu_set_beacon_filter`
//! with `enable` false), so a listening radio hears every network's.

pub const LEN: usize = 68;
/// `MT7921_FILTER_ENABLE`.
const FILTER_ENABLE: u32 = 1 << 31;
/// `MT7921_FIF_BIT_CLR`.
const BIT_CLR: u8 = 1 << 1;
/// `MT_WF_RFCR_DROP_OTHER_BEACON`.
const DROP_OTHER_BEACON: u32 = 1 << 11;

fn body(mode: u8, fif: u32, bits: u32, op: u8) -> [u8; LEN] {
    let mut b = [0u8; LEN];
    b[4] = mode;
    b[8..12].copy_from_slice(&fif.to_le_bytes());
    b[12..16].copy_from_slice(&bits.to_le_bytes());
    b[16] = op;
    b
}

/// The filter word mac80211's default flags give.
pub fn default() -> [u8; LEN] {
    body(1, FILTER_ENABLE, 0, 0)
}

/// Other networks' beacons let through.
pub fn hear_beacons() -> [u8; LEN] {
    body(2, 0, DROP_OTHER_BEACON, BIT_CLR)
}
