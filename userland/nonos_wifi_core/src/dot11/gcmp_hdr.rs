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

//! The GCMP header (IEEE Std 802.11-2020, 12.5.5.2: CCMP's layout, PN0,
//! PN1, a reserved octet, the key-id octet with ExtIV, PN2 to PN5) and the
//! GCMP nonce (12.5.5.3.3).

use super::ccmp::{aad_nonce, FrameView, CCMP_HDR_LEN};
use crate::gcmp::gcm::TAG_LEN;

pub const GCMP_HDR_LEN: usize = CCMP_HDR_LEN;
pub const GCMP_MIC_LEN: usize = TAG_LEN;
pub const EXT_IV: u8 = 0x20;

/// The PN and key index of a protected frame, if it holds a GCMP header
/// with ExtIV set and room for the MIC.
pub fn pn_and_key(frame: &[u8], hdr_len: usize) -> Option<(u64, u8)> {
    let end = hdr_len.checked_add(GCMP_HDR_LEN)?;
    if frame.len() < end.checked_add(GCMP_MIC_LEN)? || frame[hdr_len + 3] & EXT_IV == 0 {
        return None;
    }
    let h = &frame[hdr_len..end];
    let pn =
        [h[0], h[1], h[4], h[5], h[6], h[7]].iter().rev().fold(0u64, |p, &b| p << 8 | b as u64);
    Some((pn, h[3] >> 6))
}

/// The nonce: CCMP's without its first (flags and priority) octet.
pub fn nonce(frame: &[u8], v: &FrameView, pn: u64, aad: &mut [u8; 32]) -> ([u8; 12], usize) {
    let (n13, len) = aad_nonce(frame, v, pn, aad);
    let mut n = [0u8; 12];
    n.copy_from_slice(&n13[1..]);
    (n, len)
}
