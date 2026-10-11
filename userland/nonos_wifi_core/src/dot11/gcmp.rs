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

//! GCMP-256 on an MPDU (IEEE Std 802.11-2020, 12.5.5). The GCMP header has
//! the CCMP header's layout (12.5.5.2), and the AAD is built as CCMP builds
//! it (12.5.5.3.2), so both come from `ccmp`; the nonce is A2 followed by
//! the PN, most significant octet first, with no priority octet
//! (12.5.5.3.3); the MIC is 16 octets. A frame too short for its header,
//! GCMP header and MIC, or without ExtIV, is refused.

use alloc::vec::Vec;

use super::ccmp::{view, FrameView};
use super::gcmp_hdr::{nonce, EXT_IV};
pub use super::gcmp_hdr::{pn_and_key, GCMP_HDR_LEN, GCMP_MIC_LEN};
use crate::gcmp::gcm::{open, seal};

const FC_PROTECTED: u16 = 0x4000;

/// Protect a frame under `tk` at `pn`, key index 0.
pub fn encrypt(frame: &[u8], pn: u64, tk: &[u8; 32]) -> Option<Vec<u8>> {
    let v = view(frame)?;
    let mut out = Vec::with_capacity(frame.len() + GCMP_HDR_LEN + GCMP_MIC_LEN);
    out.extend_from_slice(&frame[..v.hdr_len]);
    out[1] |= (FC_PROTECTED >> 8) as u8;
    let p = pn.to_le_bytes();
    out.extend_from_slice(&[p[0], p[1], 0, EXT_IV, p[2], p[3], p[4], p[5]]);
    let v = FrameView { fc: v.fc | FC_PROTECTED, ..v };
    let mut aad = [0u8; 32];
    let (n, aad_len) = nonce(&out, &v, pn, &mut aad);
    out.extend_from_slice(&seal(tk, &n, &aad[..aad_len], &frame[v.hdr_len..]));
    Some(out)
}

/// The plaintext body of a protected frame, or `None` if it is malformed
/// or its MIC fails.
pub fn decrypt(frame: &[u8], tk: &[u8; 32]) -> Option<Vec<u8>> {
    let v = view(frame)?;
    let (pn, _) = pn_and_key(frame, v.hdr_len)?;
    let mut aad = [0u8; 32];
    let (n, aad_len) = nonce(frame, &v, pn, &mut aad);
    open(tk, &n, &aad[..aad_len], &frame[v.hdr_len + GCMP_HDR_LEN..])
}
