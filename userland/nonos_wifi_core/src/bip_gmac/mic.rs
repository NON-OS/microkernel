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

//! The BIP-GMAC-256 MIC (IEEE Std 802.11-2020, 12.5.4): GMAC, which is GCM
//! with no plaintext (NIST SP 800-38D, section 3), under the 256-bit key.
//! The IV is A2 || IPN with the IPN most significant octet first. The
//! authenticated data is the header's frame control, with Retry, Power
//! Management and More Data cleared because a retransmission or a power
//! change may alter them in flight, then A1, A2 and A3, then the whole frame
//! body with the MMIE's MIC field zero. The sequence control is left out so
//! a retry under a new sequence number still verifies.

use alloc::vec::Vec;

use super::mmie::MIC_LEN;
use crate::gcmp::aes256::Aes256;
use crate::gcmp::parts::{j0, tag};

/// Frame control, duration, A1, A2, A3 and sequence control.
const HEADER: usize = 24;

/// Retry, Power Management and More Data in the second frame control octet.
const FC1_MASK: u8 = !(0x08 | 0x10 | 0x20);

/// The header length of a management frame, or `None` for anything else.
/// With Order set the header carries a 4-octet HT Control field.
pub(super) fn header_len(frame: &[u8]) -> Option<usize> {
    let fc = frame.get(..2)?;
    if fc[0] & 0x0c != 0 {
        return None;
    }
    let len = HEADER + if fc[1] & 0x80 != 0 { 4 } else { 0 };
    (frame.len() >= len).then_some(len)
}

pub(super) fn nonce(a2: &[u8], ipn: u64) -> [u8; 12] {
    let mut n = [0u8; 12];
    n[..6].copy_from_slice(a2);
    n[6..].copy_from_slice(&ipn.to_be_bytes()[2..]);
    n
}

/// The MIC over `frame`, whose MMIE MIC field must already be zero.
pub(super) fn mic(key: &[u8; 32], frame: &[u8], header: usize, ipn: u64) -> [u8; MIC_LEN] {
    let mut aad = Vec::with_capacity(20 + frame.len() - header);
    aad.extend_from_slice(&[frame[0], frame[1] & FC1_MASK]);
    aad.extend_from_slice(&frame[4..22]);
    aad.extend_from_slice(&frame[header..]);
    let iv = nonce(&frame[10..16], ipn);
    tag(&Aes256::new(key), &j0(&iv), &aad, &[])
}
