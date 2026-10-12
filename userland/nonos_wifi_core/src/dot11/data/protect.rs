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

//! Apply software CCMP to a plaintext MPDU from `build_data`: set the
//! Protected bit, insert the CCMP header for packet number `pn`, and
//! replace the body with its AES-CCM ciphertext and MIC keyed by `tk`.

use super::*;

/// Apply software CCMP to a plaintext MPDU from `build_data`: set the Protected
/// bit, insert the CCMP header for packet number `pn`, and replace the body with
/// its AES-CCM ciphertext and MIC keyed by `tk`. This is the path for a chip
/// without hardware crypto; a hardware-CCMP chip never calls it. Returns `None`
/// if the MPDU is shorter than a MAC header.
pub fn protect(mpdu: &[u8], pn: u64, tk: &[u8; 16]) -> Option<Vec<u8>> {
    if mpdu.len() < MAC_HEADER_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]) | FC_PROTECTED;
    let mut a1 = [0u8; 6];
    let mut a2 = [0u8; 6];
    let mut a3 = [0u8; 6];
    a1.copy_from_slice(&mpdu[4..10]);
    a2.copy_from_slice(&mpdu[10..16]);
    a3.copy_from_slice(&mpdu[16..22]);
    let sc = u16::from_le_bytes([mpdu[22], mpdu[23]]);
    let plain = &mpdu[MAC_HEADER_LEN..];

    let mut frame = Vec::with_capacity(MAC_HEADER_LEN + CCMP_HDR_LEN + plain.len() + MIC_LEN);
    frame.extend_from_slice(&fc.to_le_bytes());
    frame.extend_from_slice(&mpdu[2..4]); // duration, preserved
    frame.extend_from_slice(&a1);
    frame.extend_from_slice(&a2);
    frame.extend_from_slice(&a3);
    frame.extend_from_slice(&mpdu[22..24]); // sequence control, preserved
    frame.extend_from_slice(&ccmp_header(pn));

    let aad = build_aad(fc, &a1, &a2, &a3, sc);
    let nonce = build_nonce(&a2, pn);
    let mut ct = alloc::vec![0u8; plain.len() + MIC_LEN];
    let n = ccm_encrypt(tk, &nonce, &aad, plain, &mut ct)?;
    frame.extend_from_slice(&ct[..n]);
    Some(frame)
}

/// Encrypt an ethernet frame into an 802.11 data frame addressed to the AP
/// (ToDS) at packet number `pn`, the software CCMP round trip. Equivalent to
/// `build_data` followed by `protect`; a hardware-CCMP chip uses `build_data`
/// alone. Returns `None` if the ethernet frame is malformed.
pub fn encap(
    eth: &[u8],
    our_mac: MacAddr,
    bssid: MacAddr,
    pn: u64,
    tk: &[u8; 16],
    seq: u16,
) -> Option<Vec<u8>> {
    protect(&build_data(eth, our_mac, bssid, seq)?, pn, tk)
}
