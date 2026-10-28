/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The station record's TLVs, laid out as the structs of Linux v6.12
//! `mt76_connac_mcu.h` (`sta_rec_basic`, `sta_rec_phy`, `sta_rec_ra_info`,
//! `sta_rec_state`) and filled as `mt76_connac_mcu_sta_basic_tlv` and
//! `mt76_connac_mcu_sta_tlv` fill them for an access point seen from a
//! station.

use alloc::vec::Vec;

use super::sta::{Peer, State};

pub(super) fn tlv(out: &mut Vec<u8>, tag: u16, len: u16) -> usize {
    let at = out.len();
    out.extend_from_slice(&tag.to_le_bytes());
    out.extend_from_slice(&len.to_le_bytes());
    out.resize(at + len as usize, 0);
    at
}

pub fn basic(out: &mut Vec<u8>, p: &Peer, enable: Option<State>) {
    let t = tlv(out, 0, 20);
    let conn = 2u32 | 1 << 16; // CONNECTION_INFRA_AP
    out[t + 4..t + 8].copy_from_slice(&conn.to_le_bytes());
    // CONN_STATE_PORT_SECURE when enabled, else CONN_STATE_DISCONNECT;
    // EXTRA_INFO_NEW on a record made, not on the one association updates.
    let (state, extra) = match enable {
        Some(State::None) => (2u8, 3u16),
        Some(State::Assoc) => (2, 1),
        None => (0, 1),
    };
    out[t + 8] = state;
    out[t + 10..t + 12].copy_from_slice(&p.aid.to_le_bytes());
    out[t + 12..t + 18].copy_from_slice(&p.addr);
    out[t + 18..t + 20].copy_from_slice(&extra.to_le_bytes());
}

pub fn phy(out: &mut Vec<u8>, p: &Peer) {
    let t = tlv(out, 0x15, 12);
    out[t + 4..t + 6].copy_from_slice(&p.rates.1.to_le_bytes());
    // PHY_TYPE_BIT_OFDM on 5 GHz, HR_DSSS and ERP on 2.4; RCPI 220, the
    // value Linux sends before it has averaged a signal.
    (out[t + 6], out[t + 9]) = (if p.ghz5 { 1 << 3 } else { 0x3 }, 220);
}

pub fn ra(out: &mut Vec<u8>, p: &Peer) {
    let t = tlv(out, 1, 16);
    let s = p.rates.0;
    // RA_LEGACY_OFDM at bits 13:6, RA_LEGACY_CCK at 3:0.
    let legacy = if p.ghz5 { (s & 0xff) << 6 } else { (s >> 4 & 0xff) << 6 | (s & 0xf) };
    out[t + 4..t + 6].copy_from_slice(&legacy.to_le_bytes());
}

pub fn state(out: &mut Vec<u8>, s: State) {
    let t = tlv(out, 7, 12);
    out[t + 8] = s as u8;
}
