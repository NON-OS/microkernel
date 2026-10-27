/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The station interface on the firmware: its own MAC entry and BSS, the
//! access point's station record, channel requests and the receive filter,
//! each sent as the Linux v6.12 mac80211 callback named beside it sends
//! it. One interface, index 0, as `mt7921_add_interface` gives the first.

pub mod bss;
pub mod dev;
pub mod filter;
pub mod own;
pub mod roc;
pub mod sta;
mod sta_tlv;
mod wtbl;

use crate::card::Card;
use crate::mcu::cmd::{BSS_INFO_UPDATE, DEV_INFO_UPDATE, SET_RX_FILTER};

pub const BSS_IDX: u8 = 0;
pub const OMAC_IDX: u8 = 0;
/// The interface's own WTBL entry, `MT792x_WTBL_RESERVED - idx`.
pub const OWN_WCID: u16 = 19;
/// The access point's entry, the first `mt76_wcid_alloc` hands a station.
pub const AP_WCID: u16 = 1;

/// The interface up under `addr` (`mt76_connac_mcu_uni_add_dev`, enable):
/// the MAC entry, then the BSS, unconnected.
pub fn add(c: &mut Card, addr: [u8; 6]) -> bool {
    let basic = bss::Basic { active: true, ..Default::default() };
    let (b, n) = bss::basic(basic, None);
    c.cmd(DEV_INFO_UPDATE, &dev::body(true, addr), true) && c.cmd(BSS_INFO_UPDATE, &b[..n], true)
}

/// The interface down (the same call, disable): the BSS, then the entry.
pub fn remove(c: &mut Card, addr: [u8; 6]) -> bool {
    let (b, n) = bss::basic(bss::Basic::default(), None);
    c.cmd(BSS_INFO_UPDATE, &b[..n], true) && c.cmd(DEV_INFO_UPDATE, &dev::body(false, addr), true)
}

/// The filter as `mt7921_configure_filter` leaves it, and other networks'
/// beacons heard (`mt7921_mcu_set_beacon_filter`, disabled).
pub fn listen(c: &mut Card) -> bool {
    c.cmd(SET_RX_FILTER, &filter::default(), false)
        && c.cmd(SET_RX_FILTER, &filter::hear_beacons(), false)
}
