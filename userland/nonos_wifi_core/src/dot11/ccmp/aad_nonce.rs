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

//! The AAD (IEEE Std 802.11-2020, 12.5.3.3.3) and nonce (12.5.3.3.4) of a
//! CCMP MPDU. GCMP's AAD is the same and its nonce drops the flags octet
//! (12.5.5.3.2, 12.5.5.3.3), so `gcmp` takes both from here.

use super::*;

/// Bits a retry or a power-save change flips, zeroed so the MIC survives.
const FC_MUTABLE: u16 = FC_RETRY | FC_PWR_MGT | FC_MORE_DATA;
const FC_DATA_SUBTYPE: u16 = 0x0070;
const NONCE_MANAGEMENT: u8 = 0x10;

/// Frame Control as the AAD carries it: mutable bits zeroed, data subtype
/// and (QoS data) Order zeroed, Protected set.
fn aad_fc(fc: u16, qos: bool) -> u16 {
    let mut zero = FC_MUTABLE;
    if fc_type(fc) == TYPE_DATA {
        zero |= FC_DATA_SUBTYPE;
        if qos {
            zero |= FC_ORDER;
        }
    }
    (fc & !zero) | FC_PROTECTED
}

/// Flags (the TID for QoS data, zero for other data, Management for a
/// management frame), A2, then PN5 down to PN0.
fn nonce(frame: &[u8], v: &FrameView, pn: u64) -> [u8; 13] {
    let mut n = [0u8; 13];
    let data = fc_type(v.fc) == TYPE_DATA;
    n[0] = if !data { NONCE_MANAGEMENT } else { v.qos_at.map_or(0, |_| v.tid) };
    n[1..7].copy_from_slice(&frame[10..16]);
    n[7..13].copy_from_slice(&pn.to_be_bytes()[2..8]);
    n
}

/// The nonce, and the AAD written into `aad` with its length: Frame
/// Control, A1 to A3, Sequence Control with only the fragment number kept,
/// A4 when both DS bits are set, and QoS Control with only the TID kept.
pub fn aad_nonce(frame: &[u8], v: &FrameView, pn: u64, aad: &mut [u8; 32]) -> ([u8; 13], usize) {
    let fc = aad_fc(v.fc, v.qos_at.is_some());
    aad[0..2].copy_from_slice(&fc.to_le_bytes());
    aad[2..20].copy_from_slice(&frame[4..22]);
    aad[20] = frame[22] & 0x0F;
    aad[21] = 0;
    let mut len = 22;
    if fc & (FC_TO_DS | FC_FROM_DS) == FC_TO_DS | FC_FROM_DS {
        aad[len..len + 6].copy_from_slice(&frame[24..30]);
        len += 6;
    }
    if let Some(q) = v.qos_at {
        aad[len] = frame[q] & 0x0F;
        aad[len + 1] = 0;
        len += 2;
    }
    (nonce(frame, v, pn), len)
}
