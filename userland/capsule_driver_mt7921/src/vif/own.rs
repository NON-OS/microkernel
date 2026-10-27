/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What an association sends, in Linux v6.12's order: the BSS connected,
//! with its QoS and channel (`mt76_connac_mcu_uni_add_bss`, from
//! `mt7921_mac_sta_event`), the access point's record in the associated
//! state, then the interface's own entry pointed at the BSS
//! (`mt7921_bss_info_changed`, `BSS_CHANGED_ASSOC`: `mt7921_mcu_sta_update`
//! with no station, so no basic TLV and the MUAR index 0xe). A departure
//! sends the record removed and the BSS disconnected
//! (`mt7921_mac_sta_remove`), and resets the own entry between, so no key
//! outlives the association (Linux leaves that to the next association's
//! reset of the entry).

use alloc::vec::Vec;

use super::bss::{basic, rlm, Basic};
use super::sta::{body, Peer, State};
use super::wtbl::wtbl;
use super::{BSS_IDX, OWN_WCID};
use crate::card::Card;
use crate::mcu::cmd::{BSS_INFO_UPDATE, STA_REC_UPDATE};

/// The own entry's record: the request header (MUAR 0xe), one TLV; reset
/// and pointed at `bssid`, or only reset, which takes its keys.
fn own_body(bssid: [u8; 6], set: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(&[BSS_IDX, OWN_WCID as u8, 1, 0, 1, 0xe, 0, 0]);
    let me = Peer { addr: bssid, wcid: OWN_WCID, aid: 0, ghz5: false, rates: (0, 0) };
    wtbl(&mut out, &me, set, 0xe);
    out
}

/// The join's channel and what association settled.
pub struct Joined {
    pub peer: Peer,
    pub basic: Basic,
    pub channel: u8,
    pub antennas: u8,
}

pub fn associate(c: &mut Card, j: &Joined) -> bool {
    let (b, n) = basic(j.basic, Some(false));
    let band = u8::from(j.peer.ghz5);
    c.cmd(BSS_INFO_UPDATE, &b[..n], true)
        && c.cmd(BSS_INFO_UPDATE, &rlm(j.channel, band, j.antennas), true)
        && c.cmd(STA_REC_UPDATE, &body(&j.peer, Some(State::Assoc)), true)
        && c.cmd(STA_REC_UPDATE, &own_body(j.basic.bssid, true), true)
}

/// The access point's record for a join that has not associated yet
/// (`mt7921_mac_sta_add`, state none), or removed.
pub fn record(c: &mut Card, peer: &Peer, on: bool) -> bool {
    let state = on.then_some(State::None);
    c.cmd(STA_REC_UPDATE, &body(peer, state), true)
}

/// The record removed, with the pairwise key its entry held; the own
/// entry reset, with the group keys; the BSS disconnected.
pub fn depart(c: &mut Card, peer: &Peer, basic_was: Basic) -> bool {
    let removed = record(c, peer, false);
    let reset = c.cmd(STA_REC_UPDATE, &own_body(basic_was.bssid, false), true);
    let (b, n) = basic(Basic { connected: false, ..basic_was }, Some(false));
    removed && reset && c.cmd(BSS_INFO_UPDATE, &b[..n], true)
}
