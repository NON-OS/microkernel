/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The access point's station record, `MCU_UNI_CMD_STA_REC_UPDATE`, as
//! Linux v6.12 `mt76_connac_mcu_sta_cmd` builds it for a legacy (non-HT)
//! peer of a station interface: the request header (`is_tlv_append`),
//! `STA_REC_BASIC`; when enabled `STA_REC_PHY`, `STA_REC_RA` and
//! `STA_REC_STATE` (`mt76_connac_mcu_sta_tlv`); then `STA_REC_WTBL`
//! holding a reset-and-set WTBL request with, when enabled, its generic,
//! receive and header-translation TLVs. The shared station negotiates no
//! HT, VHT or HE, so those TLVs never appear.

use alloc::vec::Vec;

use super::sta_tlv::{basic, phy, ra, state};
use super::wtbl::wtbl;
use super::{BSS_IDX, OMAC_IDX};

/// What a record says of the peer.
#[derive(Clone, Copy)]
pub struct Peer {
    pub addr: [u8; 6],
    pub wcid: u16,
    pub aid: u16,
    pub ghz5: bool,
    /// The rate bitmaps (`mac::rate::bitmaps`): supported, basic.
    pub rates: (u16, u16),
}

/// `MT76_STA_INFO_STATE_*`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    None = 0,
    Assoc = 2,
}

/// The record's body: `Some(state)` enabled in that state, `None` removed.
pub fn body(p: &Peer, enable: Option<State>) -> Vec<u8> {
    let mut out = Vec::with_capacity(128);
    let wcid = p.wcid.to_le_bytes();
    out.extend_from_slice(&[BSS_IDX, wcid[0], 0, 0, 1, OMAC_IDX, wcid[1], 0]);
    let mut n = 1u16;
    basic(&mut out, p, enable);
    if let Some(s) = enable {
        phy(&mut out, p);
        ra(&mut out, p);
        state(&mut out, s);
        n += 3;
    }
    wtbl(&mut out, p, enable.is_some(), OMAC_IDX);
    n += 1;
    out[2..4].copy_from_slice(&n.to_le_bytes());
    out
}
