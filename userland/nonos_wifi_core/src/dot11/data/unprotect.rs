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

//! Remove software CCMP from an 802.11 data frame: verify and strip the
//! CCMP header and MIC, returning the plaintext MPDU with the Protected bit
//! cleared.

use super::*;

/// Remove software CCMP from an 802.11 data frame: verify and strip the CCMP
/// header and MIC, returning the plaintext MPDU with the Protected bit cleared.
/// A hardware-CCMP chip delivers an already-decrypted MPDU and skips this,
/// handing the frame straight to `parse_data`. Returns `None` if the frame is
/// too short or the MIC fails.
pub fn unprotect(frame: &[u8], tk: &[u8; 16]) -> Option<Vec<u8>> {
    if frame.len() < MAC_HEADER_LEN + CCMP_HDR_LEN + MIC_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    let mut a1 = [0u8; 6];
    let mut a2 = [0u8; 6];
    let mut a3 = [0u8; 6];
    a1.copy_from_slice(&frame[4..10]);
    a2.copy_from_slice(&frame[10..16]);
    a3.copy_from_slice(&frame[16..22]);
    let sc = u16::from_le_bytes([frame[22], frame[23]]);
    let pn = pn_from_header(&frame[MAC_HEADER_LEN..MAC_HEADER_LEN + CCMP_HDR_LEN]);

    let aad = build_aad(fc, &a1, &a2, &a3, sc);
    let nonce = build_nonce(&a2, pn);
    let ct = &frame[MAC_HEADER_LEN + CCMP_HDR_LEN..];
    let mut plain = alloc::vec![0u8; ct.len()];
    let n = ccm_decrypt(tk, &nonce, &aad, ct, &mut plain)?;

    let mut mpdu = Vec::with_capacity(MAC_HEADER_LEN + n);
    mpdu.extend_from_slice(&(fc & !FC_PROTECTED).to_le_bytes());
    mpdu.extend_from_slice(&frame[2..4]); // duration
    mpdu.extend_from_slice(&a1);
    mpdu.extend_from_slice(&a2);
    mpdu.extend_from_slice(&a3);
    mpdu.extend_from_slice(&frame[22..24]); // sequence control
    mpdu.extend_from_slice(&plain[..n]);
    Some(mpdu)
}
