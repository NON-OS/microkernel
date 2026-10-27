/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A frame's transmit words: the 32-byte TXD for an 802.11-format frame
//! (Linux v6.12 `mt76_connac2_mac_write_txwi` and `..._write_txwi_80211`,
//! the MT7921 family's branches), then the 32-byte hardware TXP that
//! points at the frame (`mt76_connac_write_hw_txp`). Every frame goes at a
//! fixed rate with block-ack off; none is ever fragmented or keyed by the
//! chip (the shared station seals data itself), so no `PROTECT_FRAME`.
//! The station's own sequence number is kept (`SN_VALID`), as Linux keeps
//! an injected frame's.

use nonos_wifi_core::dot11::header::{fc_subtype, fc_type, header_len};

pub const TXWI_LEN: usize = 64;

const PKT_FMT_CT: u32 = 0;
const Q_ALTX0: u32 = 0x10; // management, MT_LMAC_ALTX0
const Q_AC_BE: u32 = 1; // best effort, mt76_connac_lmac_mapping(2)
const HDR_FORMAT_80211: u32 = 2;
const REM_TX_COUNT: u32 = 15;
const MSDU_ID_VALID: u16 = 1 << 15;
const TXD_LEN_LAST: u16 = 1 << 15;

/// What the TXD carries besides the frame: the WTBL entry it is sent
/// under, the token its buffer is held by, where the chip reads the frame,
/// and the fixed rate (`mac::rate::value`).
pub struct TxSpec {
    pub wcid: u16,
    pub token: u16,
    pub frame_dev: u32,
    pub rate: u16,
}

/// The TXD and TXP for `frame`, or `None` for one too short for its own
/// header, too long for the TXP's twelve-bit length, or fragmented.
pub fn txwi(frame: &[u8], s: &TxSpec) -> Option<[u8; TXWI_LEN]> {
    let fc = u16::from_le_bytes([*frame.first()?, *frame.get(1)?]);
    let hdr = header_len(fc);
    let sc = u16::from_le_bytes([*frame.get(22)?, *frame.get(23)?]);
    if frame.len() < hdr || frame.len() > 0xfff || fc & 0x0400 != 0 || sc & 0xf != 0 {
        return None;
    }
    let (ty, sub) = (u32::from(fc_type(fc)), u32::from(fc_subtype(fc)));
    let data = ty == 2;
    let group = frame[4] & 1 != 0;
    let tid = if data && sub & 0x8 != 0 { u32::from(frame[hdr - 2] & 7) } else { 0 };
    let q = if data { Q_AC_BE } else { Q_ALTX0 };
    let w0 = (frame.len() + 32) as u32 | PKT_FMT_CT << 23 | q << 25;
    let w1 = 1 << 31 | u32::from(s.wcid & 0x3ff) | HDR_FORMAT_80211 << 16;
    let w1 = w1 | (hdr as u32 / 2) << 11 | tid << 20;
    // FIX_RATE and, with it, HTC_VLD (the chip adds no HT control).
    let w2 = sub | ty << 4 | u32::from(group) << 10 | 1 << 13 | 1 << 31;
    let seq = u32::from(sc >> 4);
    let w3 = u32::from(group) | REM_TX_COUNT << 11 | seq << 16 | 1 << 28 | 1 << 31;
    let w6 = 1 << 2 | u32::from(s.rate & 0x3fff) << 16; // FIXED_BW, TX_RATE
    let w7 = sub << 16 | ty << 20;
    let mut out = [0u8; TXWI_LEN];
    for (i, w) in [w0, w1, w2, w3, 0, 0, w6, w7].iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
    }
    out[32..34].copy_from_slice(&(s.token | MSDU_ID_VALID).to_le_bytes());
    out[40..44].copy_from_slice(&s.frame_dev.to_le_bytes());
    out[44..46].copy_from_slice(&(frame.len() as u16 | TXD_LEN_LAST).to_le_bytes());
    Some(out)
}
