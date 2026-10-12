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

//! The CCM nonce and AAD for a received frame (hostapd's `ccmp_aad_nonce`
//! is the reference).

use super::*;

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
