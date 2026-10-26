// NONOS Operating System (AGPL-3.0-or-later)

//! What power save needs from a join: the AID the access point granted, and
//! the DTIM period its beacon names.

use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::dot11::parse::{find_ie, parse_assoc_response, FIXED_FIELDS_LEN};

const IE_TIM: u8 = 5;

/// The AID of a successful association response sent from `bssid`.
pub fn granted_aid(frame: &[u8], bssid: [u8; 6]) -> Option<u16> {
    let from_bss = frame.get(10..16) == Some(&bssid[..]);
    match parse_assoc_response(frame) {
        Some((0, aid)) if from_bss => Some(aid),
        _ => None,
    }
}

/// The DTIM period of `beacon`'s TIM element; one when it carries none, or
/// names zero.
pub fn dtim_period(beacon: &[u8]) -> u8 {
    let tags = beacon.get(MAC_HEADER_LEN + FIXED_FIELDS_LEN..);
    let tim = tags.and_then(|t| find_ie(t, IE_TIM));
    tim.and_then(|t| t.get(1).copied()).filter(|&p| p != 0).unwrap_or(1)
}
