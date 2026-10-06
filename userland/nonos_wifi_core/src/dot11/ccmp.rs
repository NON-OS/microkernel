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

//! CCMP-128 on a received MPDU of any shape (IEEE Std 802.11-2020,
//! 12.5.3.3). Access points send QoS data, whose header is 26 octets (30 with
//! an HT Control field), and the CCM nonce and AAD depend on it: the nonce's
//! priority octet is the TID, the AAD masks the subtype bits and the Order
//! bit and appends the QoS Control field with only the TID kept. `data.rs`
//! frames the station's own transmissions, which are non-QoS; this reads
//! whatever the AP sent. The view refuses a frame too short for its header,
//! CCMP header and MIC, and a CCMP header without ExtIV.

use alloc::vec::Vec;

use super::header::{fc_subtype, fc_type, FC_FROM_DS, FC_TO_DS, MAC_HEADER_LEN, TYPE_DATA};
use crate::ccmp::ccm::ccm_decrypt;

/// The CCMP header and MIC lengths.
pub const CCMP_HDR_LEN: usize = 8;
pub const MIC_LEN: usize = 8;
/// Frame-control bits.
const FC_RETRY: u16 = 0x0800;
const FC_PWR_MGT: u16 = 0x1000;
const FC_MORE_DATA: u16 = 0x2000;
const FC_PROTECTED: u16 = 0x4000;
const FC_ORDER: u16 = 0x8000;
/// The QoS subtype bit (bit 7 of the frame control's first octet).
const SUBTYPE_QOS: u8 = 0x08;
/// The HT Control field a QoS frame with the Order bit carries.
const HT_CONTROL_LEN: usize = 4;
/// The ExtIV bit of the CCMP header's key-id octet.
const EXT_IV: u8 = 0x20;

/// The parts of a received frame CCMP needs, all bounds-checked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameView {
    pub fc: u16,
    /// MAC header length, QoS Control, Address 4 and HT Control included.
    pub hdr_len: usize,
    /// QoS Control field offset, for a QoS data frame.
    pub qos_at: Option<usize>,
    /// The TID (QoS priority), zero for a non-QoS frame.
    pub tid: u8,
}

/// The header layout of a received MPDU, or `None` if it is shorter than the
/// header its frame control describes.
pub fn view(frame: &[u8]) -> Option<FrameView> {
    if frame.len() < MAC_HEADER_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    let mut hdr_len = MAC_HEADER_LEN;
    if fc & FC_TO_DS != 0 && fc & FC_FROM_DS != 0 {
        hdr_len += 6;
    }
    let mut qos_at = None;
    let mut tid = 0;
    if fc_type(fc) == TYPE_DATA && fc_subtype(fc) & SUBTYPE_QOS != 0 {
        qos_at = Some(hdr_len);
        tid = *frame.get(hdr_len)? & 0x0F;
        hdr_len += 2;
        if fc & FC_ORDER != 0 {
            hdr_len += HT_CONTROL_LEN;
        }
    }
    if frame.len() < hdr_len {
        return None;
    }
    Some(FrameView { fc, hdr_len, qos_at, tid })
}

/// The packet number and key index from the CCMP header after `hdr_len`, or
/// `None` if the frame cannot hold the header and a MIC or ExtIV is clear.
pub fn pn_and_key(frame: &[u8], hdr_len: usize) -> Option<(u64, u8)> {
    let end = hdr_len.checked_add(CCMP_HDR_LEN)?;
    if frame.len() < end.checked_add(MIC_LEN)? {
        return None;
    }
    let h = &frame[hdr_len..end];
    if h[3] & EXT_IV == 0 {
        return None;
    }
    let pn = (h[0] as u64)
        | ((h[1] as u64) << 8)
        | ((h[4] as u64) << 16)
        | ((h[5] as u64) << 24)
        | ((h[6] as u64) << 32)
        | ((h[7] as u64) << 40);
    Some((pn, h[3] >> 6))
}

/// The CCM nonce and AAD for a received frame (hostapd's `ccmp_aad_nonce`
/// is the reference). Returns the AAD length used in `aad`.
pub fn aad_nonce(frame: &[u8], v: &FrameView, pn: u64, aad: &mut [u8; 32]) -> ([u8; 13], usize) {
    let mut fc = v.fc;
    let mut nonce = [0u8; 13];
    if fc_type(fc) == TYPE_DATA {
        fc &= !0x0070; // subtype bits 4-6
        if v.qos_at.is_some() {
            fc &= !FC_ORDER;
            nonce[0] = v.tid;
        }
    } else {
        nonce[0] = 0x10; // the Management flag
    }
    fc &= !(FC_RETRY | FC_PWR_MGT | FC_MORE_DATA);
    fc |= FC_PROTECTED;
    aad[0..2].copy_from_slice(&fc.to_le_bytes());
    aad[2..20].copy_from_slice(&frame[4..22]);
    let sc = u16::from_le_bytes([frame[22], frame[23]]) & 0x000F;
    aad[20..22].copy_from_slice(&sc.to_le_bytes());
    let mut n = 22;
    if fc & FC_TO_DS != 0 && fc & FC_FROM_DS != 0 {
        aad[n..n + 6].copy_from_slice(&frame[24..30]);
        n += 6;
    }
    if let Some(q) = v.qos_at {
        // TID only: EOSP, Ack Policy, A-MSDU Present and the high octet masked.
        aad[n] = frame[q] & 0x0F;
        aad[n + 1] = 0;
        n += 2;
    }
    nonce[1..7].copy_from_slice(&frame[10..16]);
    for (i, b) in nonce[7..13].iter_mut().enumerate() {
        *b = (pn >> (40 - 8 * i)) as u8;
    }
    (nonce, n)
}

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
