// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What a frame received while joining is to the state machine: management
//! frames go to it whole, an EAPOL payload is lifted out of its data frame,
//! and an EAPOL reply is wrapped into an unprotected data frame to the AP,
//! since no key exists yet.

use alloc::vec::Vec;

use crate::dot11::data::build_data;
use crate::dot11::header::{fc_type, MAC_HEADER_LEN, TYPE_DATA, TYPE_MGMT};
use crate::frame::LLC_SNAP;

const ETHERTYPE_EAPOL: [u8; 2] = [0x88, 0x8E];

#[derive(Debug, PartialEq, Eq)]
pub enum RxKind<'a> {
    Mgmt,
    Eapol(&'a [u8]),
    Other,
}

/// Classify a received frame for the join. The EAPOL signature is searched
/// for past the MAC header rather than at a computed offset, since QoS,
/// HT Control and four-address headers move it, and the handshake travels in
/// the clear.
pub fn classify(frame: &[u8]) -> RxKind<'_> {
    let Some(fc) = frame.get(..2).map(|b| u16::from_le_bytes([b[0], b[1]])) else {
        return RxKind::Other;
    };
    match fc_type(fc) {
        TYPE_MGMT => RxKind::Mgmt,
        TYPE_DATA if frame.len() >= MAC_HEADER_LEN + 8 => (MAC_HEADER_LEN..=frame.len() - 8)
            .find(|&p| frame[p..p + 6] == LLC_SNAP && frame[p + 6..p + 8] == ETHERTYPE_EAPOL)
            .map_or(RxKind::Other, |p| RxKind::Eapol(&frame[p + 8..])),
        _ => RxKind::Other,
    }
}

/// An EAPOL payload framed as an unprotected data frame to the AP.
pub fn wrap_eapol(eapol: &[u8], our_mac: [u8; 6], bssid: [u8; 6]) -> Option<Vec<u8>> {
    let mut eth = Vec::with_capacity(14 + eapol.len());
    eth.extend_from_slice(&bssid);
    eth.extend_from_slice(&our_mac);
    eth.extend_from_slice(&ETHERTYPE_EAPOL);
    eth.extend_from_slice(eapol);
    build_data(&eth, our_mac, bssid, 0)
}
