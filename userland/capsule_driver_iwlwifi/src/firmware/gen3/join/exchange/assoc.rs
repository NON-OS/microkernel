// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The AID and capability an association response gives the station.

use nonos_wifi_core::dot11::parse::parse_assoc_response;

/// Where the association response's capability field starts.
const MAC_HEADER: usize = 24;
const MAX_AID: u16 = 2007;

// The AID and capability of an association response. An AID outside 1 to
// 2007 (`IEEE80211_MAX_AID`) is taken as none, as mac80211 takes it.
pub fn association(frame: &[u8]) -> (u16, u16) {
    let aid = parse_assoc_response(frame).map_or(0, |(_, aid)| aid);
    let aid = if aid > MAX_AID { 0 } else { aid };
    let cap = frame.get(MAC_HEADER..MAC_HEADER + 2).map_or(0, |c| u16::from_le_bytes([c[0], c[1]]));
    (aid, cap)
}
