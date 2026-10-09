// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The access point's handshake in the clear: the EAPOL payload taken out of
//! a received data frame, and a reply put into one.

use alloc::vec::Vec;

use nonos_wifi_core::dot11::ccmp::view;
use nonos_wifi_core::dot11::data::build_data;
use nonos_wifi_core::dot11::header::TYPE_DATA;
use nonos_wifi_core::frame::LLC_SNAP;

const ETHERTYPE_EAPOL: [u8; 2] = [0x88, 0x8E];

/// this station in the clear, or `None`.
pub fn eapol_payload<'f>(frame: &'f [u8], us: &[u8; 6], bssid: &[u8; 6]) -> Option<&'f [u8]> {
    let v = view(frame)?;
    if (v.fc >> 2) & 0x3 != u16::from(TYPE_DATA) || v.fc & 0x0300 != 0x0200 || v.fc & 0x4000 != 0 {
        return None;
    }
    if frame[4..10] != us[..] || frame[10..16] != bssid[..] {
        return None;
    }
    let body = frame.get(v.hdr_len..)?;
    (body.len() >= 8 && body[..6] == LLC_SNAP && body[6..8] == ETHERTYPE_EAPOL).then(|| &body[8..])
}

/// An EAPOL payload to the access point, in an unprotected data frame.
pub fn eapol_frame(eapol: &[u8], us: [u8; 6], bssid: [u8; 6], seq: u16) -> Option<Vec<u8>> {
    let mut eth = Vec::with_capacity(14 + eapol.len());
    eth.extend_from_slice(&bssid);
    eth.extend_from_slice(&us);
    eth.extend_from_slice(&ETHERTYPE_EAPOL);
    eth.extend_from_slice(eapol);
    build_data(&eth, us, bssid, seq)
}
