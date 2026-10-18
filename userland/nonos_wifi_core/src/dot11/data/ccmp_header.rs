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

//! The CCMP header, nonce and AAD, built exactly as 802.11 12.5.3 lays
//! them out.

use super::*;

// The 8-octet CCMP header: PN0, PN1, reserved, key-id byte (ExtIV set), then
// PN2..PN5. PN0 is the least significant octet.
pub fn ccmp_header(pn: u64) -> [u8; 8] {
    [
        pn as u8,
        (pn >> 8) as u8,
        0,
        0x20, // ExtIV, key id 0
        (pn >> 16) as u8,
        (pn >> 24) as u8,
        (pn >> 32) as u8,
        (pn >> 40) as u8,
    ]
}

pub(super) fn pn_from_header(h: &[u8]) -> u64 {
    (h[0] as u64)
        | ((h[1] as u64) << 8)
        | ((h[4] as u64) << 16)
        | ((h[5] as u64) << 24)
        | ((h[6] as u64) << 32)
        | ((h[7] as u64) << 40)
}

/// The 13-octet CCM nonce: a priority octet (zero for non-QoS data), the
/// transmitter address, then the 48-bit packet number most significant octet
/// first.
pub fn build_nonce(a2: &MacAddr, pn: u64) -> [u8; 13] {
    let mut n = [0u8; 13];
    n[1..7].copy_from_slice(a2);
    n[7] = (pn >> 40) as u8;
    n[8] = (pn >> 32) as u8;
    n[9] = (pn >> 24) as u8;
    n[10] = (pn >> 16) as u8;
    n[11] = (pn >> 8) as u8;
    n[12] = pn as u8;
    n
}

/// The 22-octet AAD for a non-QoS data frame: the frame-control field with the
/// retry, power-management, more-data and order subfields masked to zero, the
/// three addresses, and the sequence-control field with the sequence number
/// masked out (only the fragment number retained).
pub fn build_aad(fc: u16, a1: &MacAddr, a2: &MacAddr, a3: &MacAddr, sc: u16) -> [u8; 22] {
    let fc_masked = fc & !(0x0800 | 0x1000 | 0x2000 | 0x8000);
    let sc_masked = sc & 0x000F;
    let mut aad = [0u8; 22];
    aad[0..2].copy_from_slice(&fc_masked.to_le_bytes());
    aad[2..8].copy_from_slice(a1);
    aad[8..14].copy_from_slice(a2);
    aad[14..20].copy_from_slice(a3);
    aad[20..22].copy_from_slice(&sc_masked.to_le_bytes());
    aad
}
