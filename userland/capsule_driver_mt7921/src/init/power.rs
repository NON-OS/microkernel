/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The transmit power limits `__mt7921_start` gives the firmware before the
//! radio is used (`mt7921_set_tx_sar_pwr`, `mt76_connac_mcu_set_rate_txpower`
//! of Linux v6.12), as Linux sends them with no device tree, no SAR and no
//! ACPI table: `MCU_CE_CMD_SET_RATE_TX_POWER`, per band, eight channels a
//! message, each a 44-byte header (`struct mt76_connac_tx_power_limit_tlv`:
//! the channel count, the band, whether it is the last, `alpha2`) and per
//! channel its number and 161 limits in half-dBm (`mt76_connac_mcu_build_sku`
//! for the MT7921 family). A channel the world domain allows is held to its
//! 20 dBm (`country 00`), any other to 127, Linux's "no limit". The CLC tables
//! are not sent, as Linux runs with `disable_clc`: the firmware's defaults.

use alloc::vec::Vec;

use nonos_wifi_core::regdom::{Band, Permit, CHANNELS_2GHZ, CHANNELS_5GHZ, WORLD};

use super::caps::Caps;

const HDR: usize = 44;
const SKU: usize = 161;
const BATCH: usize = 8;
const NONE: u8 = 127;
const WORLD_LIMIT: u8 = 2 * 20;
const LIST_2G: [u8; 14] = CHANNELS_2GHZ;
const LIST_5G: [u8; 48] = [
    36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 100, 102, 104, 106, 108, 110, 112,
    114, 116, 118, 120, 122, 124, 126, 128, 132, 134, 136, 138, 140, 142, 144, 149, 151, 153, 155,
    157, 159, 161, 165, 169, 173, 177,
];

/// One channel's limits: CCK (none on 5 GHz), OFDM, HT, VHT (ten of each
/// twelve), HE, every one at `target`.
fn sku(target: u8, ghz5: bool) -> [u8; SKU] {
    let mut s = [target; SKU];
    if ghz5 {
        s[..4].fill(NONE);
    }
    (0..4).for_each(|g| s[29 + 12 * g + 10..29 + 12 * g + 12].fill(NONE));
    s
}

fn limit(band: Band, ch: u8) -> u8 {
    let ours = if band == Band::Ghz5 { &CHANNELS_5GHZ[..] } else { &CHANNELS_2GHZ[..] };
    let allowed = ours.contains(&ch) && WORLD.permit(band, ch) != Permit::Disabled;
    if allowed {
        WORLD_LIMIT
    } else {
        NONE
    }
}

/// Every message's body for a card with `caps`, in Linux's order.
pub fn bodies(caps: Caps) -> Vec<Vec<u8>> {
    let last = if caps.ghz5 { 177 } else { 14 };
    let bands = [(caps.ghz2, Band::Ghz2, &LIST_2G[..]), (caps.ghz5, Band::Ghz5, &LIST_5G[..])];
    let mut out = Vec::new();
    for (_, band, list) in bands.into_iter().filter(|b| b.0) {
        for batch in list.chunks(BATCH) {
            let mut b = alloc::vec![0u8; HDR];
            (b[4], b[5], b[8], b[9]) =
                (batch.len() as u8, 1 + (band == Band::Ghz5) as u8, b'0', b'0');
            b[6] = u8::from(batch.last() == Some(&last));
            for &ch in batch {
                b.push(ch);
                b.extend_from_slice(&sku(limit(band, ch), band == Band::Ghz5));
            }
            out.push(b);
        }
    }
    out
}
