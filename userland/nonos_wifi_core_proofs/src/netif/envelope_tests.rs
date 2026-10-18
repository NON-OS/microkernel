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

//! Proofs for the net_core link adapter: given a net_core request, the WiFi
//! driver produces a response in exactly the shape net_core parses. The four
//! operations, plus the "link down" and "no frame" cases, are checked against a
//! modeled link so the binding is correct without a radio. The frame handed to
//! the link on transmit and returned from it on receive are the Ethernet frames
//! net_core exchanges; the 802.11/CCMP translation is the LinkPort's job.
//!
//! The envelope helpers are here; the operations are in `netif::mac_tests` and
//! `netif::packet_tests`.

use crate::netif::link::MockLink;
use nonos_wifi_core::netif::serve::{serve, MAX_RESPONSE};
use nonos_wifi_core::netif::wire::{HDR_LEN, MAGIC_NNET, OP_LINK_STATUS, VERSION};

pub fn request(op: u16, rid: u32, payload: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8; HDR_LEN + payload.len()];
    v[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    v[4..6].copy_from_slice(&VERSION.to_le_bytes());
    v[6..8].copy_from_slice(&op.to_le_bytes());
    v[12..16].copy_from_slice(&rid.to_le_bytes());
    v[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    v[HDR_LEN..].copy_from_slice(payload);
    v
}

// Parse a response the way net_core does: verify the envelope and return
// (op, request_id, payload_len).
pub fn parse(resp: &[u8]) -> (u16, u32, u32) {
    assert_eq!(u32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]), MAGIC_NNET);
    assert_eq!(u16::from_le_bytes([resp[4], resp[5]]), VERSION);
    let op = u16::from_le_bytes([resp[6], resp[7]]);
    let rid = u32::from_le_bytes([resp[12], resp[13], resp[14], resp[15]]);
    let plen = u32::from_le_bytes([resp[16], resp[17], resp[18], resp[19]]);
    (op, rid, plen)
}

pub fn status(resp: &[u8]) -> i32 {
    i32::from_le_bytes([resp[HDR_LEN], resp[HDR_LEN + 1], resp[HDR_LEN + 2], resp[HDR_LEN + 3]])
}

#[test]
fn a_malformed_request_is_rejected() {
    let mut link = MockLink::up_with([0; 6]);
    let mut out = [0u8; MAX_RESPONSE];
    // Wrong magic.
    let mut bad = request(OP_LINK_STATUS, 1, &[]);
    bad[0] ^= 0xFF;
    assert!(serve(&bad, &mut link, &mut out).is_none());
    // Truncated header.
    assert!(serve(&[0u8; 8], &mut link, &mut out).is_none());
}
