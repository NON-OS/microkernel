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

//! Protect a plaintext frame (data or a robust management frame) under `tk`
//! at packet number `pn`, key index 0: set the Protected bit, insert the
//! CCMP header and replace the body with ciphertext and MIC.

use super::*;

/// Protect a plaintext frame (data or a robust management frame) under `tk`
/// at packet number `pn`, key index 0: set the Protected bit, insert the CCMP
/// header and replace the body with ciphertext and MIC. The nonce and AAD are
/// those `decrypt` checks, so a management frame gets the Management nonce
/// flag (IEEE Std 802.11-2020, 12.5.3.3.4). `None` for a malformed frame.
pub fn encrypt(frame: &[u8], pn: u64, tk: &[u8; 16]) -> Option<Vec<u8>> {
    let v = view(frame)?;
    let mut out = Vec::with_capacity(frame.len() + CCMP_HDR_LEN + MIC_LEN);
    out.extend_from_slice(&frame[..v.hdr_len]);
    out[1] |= (FC_PROTECTED >> 8) as u8;
    let p = pn.to_be_bytes();
    out.extend_from_slice(&[p[7], p[6], 0, EXT_IV, p[5], p[4], p[3], p[2]]);
    let v = FrameView { fc: v.fc | FC_PROTECTED, ..v };
    let mut aad = [0u8; 32];
    let (nonce, aad_len) = aad_nonce(&out, &v, pn, &mut aad);
    let body = &frame[v.hdr_len..];
    let mut ct = alloc::vec![0u8; body.len() + MIC_LEN];
    let n = crate::ccmp::ccm::ccm_encrypt(tk, &nonce, &aad[..aad_len], body, &mut ct)?;
    out.extend_from_slice(&ct[..n]);
    Some(out)
}

/// Decrypt a protected frame under `tk`: the plaintext body after the CCMP
/// header, without the MIC. `None` if the frame is malformed or the MIC fails.
pub fn decrypt(frame: &[u8], tk: &[u8; 16]) -> Option<Vec<u8>> {
    let v = view(frame)?;
    let (pn, _) = pn_and_key(frame, v.hdr_len)?;
    let mut aad = [0u8; 32];
    let (nonce, aad_len) = aad_nonce(frame, &v, pn, &mut aad);
    let ct = &frame[v.hdr_len + CCMP_HDR_LEN..];
    let mut plain = alloc::vec![0u8; ct.len()];
    let n = ccm_decrypt(tk, &nonce, &aad[..aad_len], ct, &mut plain)?;
    plain.truncate(n);
    Some(plain)
}
