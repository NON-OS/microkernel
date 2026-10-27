/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The channel domain told to the firmware (`MCU_CE_CMD_SET_CHAN_DOMAIN`,
//! Linux v6.12 `mt76_connac_mcu_set_channel_domain`): a 12-byte header
//! (`alpha2`, the bandwidth per band, the channel count per band), then
//! eight bytes per channel, its number and its `IEEE80211_CHAN_*` flags.
//! The channels and their flags are the world domain's that the shared
//! station scans under (`nonos_wifi_core::regdom::WORLD`): a channel it
//! disables is left out, one it holds passive is `NO_IR` (and `RADAR` if
//! it is a radar channel), as cfg80211 marks them.

use alloc::vec::Vec;

use nonos_wifi_core::regdom::{Band, Permit, CHANNELS_2GHZ, CHANNELS_5GHZ, WORLD};

use super::caps::Caps;

const CHAN_NO_IR: u32 = 1 << 1;
const CHAN_RADAR: u32 = 1 << 3;
/// `BW_20_40M` for 2.4 GHz and `BW_20_40_80_160M` for 5 GHz, Linux's.
const BW_2G: u8 = 0;
const BW_5G: u8 = 3;

fn put(out: &mut Vec<u8>, band: Band, channels: &[u8]) -> u8 {
    let mut n = 0u8;
    for &ch in channels {
        let flags = match WORLD.permit(band, ch) {
            Permit::Disabled => continue,
            Permit::Passive { dfs } => CHAN_NO_IR | if dfs { CHAN_RADAR } else { 0 },
            Permit::Active => 0,
        };
        out.extend_from_slice(&u16::from(ch).to_le_bytes());
        out.extend_from_slice(&[0, 0]);
        out.extend_from_slice(&flags.to_le_bytes());
        n += 1;
    }
    n
}

/// The command's body for a card with `caps`.
pub fn body(caps: Caps) -> Vec<u8> {
    let mut out = Vec::with_capacity(12 + 8 * 42);
    out.extend_from_slice(&[b'0', b'0', 0, 0, BW_2G, BW_5G, BW_5G, 0, 0, 0, 0, 0]);
    let n2 = if caps.ghz2 { put(&mut out, Band::Ghz2, &CHANNELS_2GHZ) } else { 0 };
    let n5 = if caps.ghz5 { put(&mut out, Band::Ghz5, &CHANNELS_5GHZ) } else { 0 };
    (out[8], out[9]) = (n2, n5);
    out
}
