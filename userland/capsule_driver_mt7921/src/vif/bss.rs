/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! `MCU_UNI_CMD_BSS_INFO_UPDATE` bodies, Linux v6.12: the 4-byte header
//! (`bss_idx`), then `struct mt76_connac_bss_basic_tlv` (tag 0, 32 bytes)
//! as `mt76_connac_mcu_uni_add_dev` and `mt76_connac_mcu_uni_add_bss`
//! fill it for a station, the latter followed by the QoS TLV (tag 15);
//! and the channel's RLM TLV (tag 2) of `mt76_connac_mcu_uni_set_chctx`.
//! In this command a connection state of 0 is connected, 1 not.

use super::{BSS_IDX, OMAC_IDX, OWN_WCID};

/// `CONNECTION_INFRA_STA`: `STA_TYPE_STA | NETWORK_INFRA`.
const CONN_INFRA_STA: u32 = 1 | 1 << 16;

/// What the basic TLV says beyond the station's fixed indices.
#[derive(Clone, Copy, Default)]
pub struct Basic {
    pub active: bool,
    pub connected: bool,
    pub bssid: [u8; 6],
    pub beacon_int: u16,
    pub dtim: u8,
    /// `mt76_connac_get_phy_mode` and `..._v2` for the band.
    pub phymode: u8,
    pub nonht_basic_phy: u8,
}

pub fn basic(b: Basic, qos: Option<bool>) -> ([u8; 44], usize) {
    let mut out = [0u8; 44];
    out[0] = BSS_IDX;
    let t = &mut out[4..36];
    t[2..4].copy_from_slice(&32u16.to_le_bytes());
    (t[4], t[5], t[6], t[7]) = (u8::from(b.active), OMAC_IDX, OMAC_IDX, 0);
    t[8..12].copy_from_slice(&CONN_INFRA_STA.to_le_bytes());
    (t[12], t[13]) = (u8::from(!b.connected), 0);
    t[14..20].copy_from_slice(&b.bssid);
    t[20..22].copy_from_slice(&OWN_WCID.to_le_bytes());
    t[22..24].copy_from_slice(&b.beacon_int.to_le_bytes());
    (t[24], t[25]) = (b.dtim, b.phymode);
    t[26..28].copy_from_slice(&OWN_WCID.to_le_bytes());
    t[28..30].copy_from_slice(&u16::from(b.nonht_basic_phy).to_le_bytes());
    let Some(qos) = qos else { return (out, 36) };
    out[36..38].copy_from_slice(&15u16.to_le_bytes()); // UNI_BSS_INFO_QBSS
    out[38..40].copy_from_slice(&8u16.to_le_bytes());
    out[40] = u8::from(qos);
    (out, 44)
}

/// The RLM TLV for a 20 MHz channel: control and centre the same, no
/// secondary, short slot, `band` as nl80211 numbers it (0 is 2.4 GHz).
pub fn rlm(channel: u8, band: u8, antennas: u8) -> [u8; 20] {
    let mut out = [0u8; 20];
    out[0] = BSS_IDX;
    out[4..6].copy_from_slice(&2u16.to_le_bytes()); // UNI_BSS_INFO_RLM
    out[6..8].copy_from_slice(&16u16.to_le_bytes());
    (out[8], out[9], out[11]) = (channel, channel, 0); // CMD_CBW_20MHZ
    (out[12], out[13], out[14]) = (antennas.count_ones() as u8, antennas, 1);
    out[17] = band;
    out
}
