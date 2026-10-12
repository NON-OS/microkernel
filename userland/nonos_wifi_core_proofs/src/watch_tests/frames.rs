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

//! The access point's management frames, in the clear, under the pairwise key
//! or under BIP, and a reading of the SA Query the station sends back. BIP is
//! signed here by hand from RFC 4493's CMAC, apart from the station's check.

use super::sim::{AP, TK};
use nonos_wifi_core::ccmp::cmac::aes_cmac;
use nonos_wifi_core::dot11::action::{parse_sa_query, SaQuery};
use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt};

pub const DEAUTH: u8 = 12;
pub const DISASSOC: u8 = 10;
pub const BEACON: u8 = 8;
pub const PROBE_RESPONSE: u8 = 5;
pub const ACTION: u8 = 13;

/// A management frame of `subtype` from `from` to `to` with `body`.
pub fn mgmt_from(from: [u8; 6], subtype: u8, to: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![subtype << 4, 0, 0, 0];
    f.extend_from_slice(&to);
    f.extend_from_slice(&from);
    f.extend_from_slice(&from);
    f.extend_from_slice(&[0x20, 0x01]);
    f.extend_from_slice(body);
    f
}

/// The same from the access point.
pub fn mgmt(subtype: u8, to: [u8; 6], body: &[u8]) -> Vec<u8> {
    mgmt_from(AP, subtype, to, body)
}

/// A frame protected under `TK` with packet number `pn`.
pub fn protected(frame: &[u8], pn: u64) -> Vec<u8> {
    encrypt(frame, pn, &TK).unwrap()
}

/// A frame to all with an MME under `igtk`: `key_id`, `ipn` and the MIC over
/// the masked frame control, the three addresses and the body.
pub fn bip_signed(frame: &[u8], key_id: u16, ipn: u64, igtk: &[u8; 16]) -> Vec<u8> {
    let mut f = frame.to_vec();
    f.extend_from_slice(&[76, 16]);
    f.extend_from_slice(&key_id.to_le_bytes());
    f.extend_from_slice(&ipn.to_le_bytes()[..6]);
    let fc = u16::from_le_bytes([f[0], f[1]]) & !0x3800;
    let mut input = fc.to_le_bytes().to_vec();
    input.extend_from_slice(&f[4..22]);
    input.extend_from_slice(&f[24..]);
    input.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&aes_cmac(igtk, &input)[..8]);
    f
}

/// The SA Query in a frame the station sent, read under `TK`.
pub fn sa_query_of(frame: &[u8]) -> Option<SaQuery> {
    assert_eq!(frame[4..10], AP, "sent to the access point");
    parse_sa_query(&decrypt(frame, &TK)?)
}
