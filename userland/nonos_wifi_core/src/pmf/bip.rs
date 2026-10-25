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

//! BIP-CMAC-128 (IEEE Std 802.11-2020, 12.5.4): the integrity check on
//! group-addressed robust management frames, an AES-CMAC under the IGTK over
//! the masked frame control, the three addresses and the body, with the MMIE's
//! MIC field zeroed, truncated to 64 bits. The sequence control is left out,
//! so a retransmission verifies.

use alloc::vec::Vec;

use super::mmie::{Mmie, MMIE_LEN};
use crate::ccmp::cmac::aes_cmac_parts;
use crate::dot11::header::{FC_PROTECTED, MAC_HEADER_LEN};

// Retry, Power Management and More Data change in flight.
const FC_MASK: u16 = !0x3800;

/// The Order bit: a management frame with it set carries an HT Control field
/// after Address 3, which this check does not take.
const FC_ORDER: u16 = 0x8000;

fn mic(igtk: &[u8; 16], frame: &[u8]) -> [u8; 8] {
    let fc = u16::from_le_bytes([frame[0], frame[1]]) & FC_MASK;
    let body = &frame[MAC_HEADER_LEN..];
    let (head, _) = body.split_at(body.len() - 8);
    let full = aes_cmac_parts(igtk, &[&fc.to_le_bytes(), &frame[4..22], head, &[0u8; 8]]);
    let mut m = [0u8; 8];
    m.copy_from_slice(&full[..8]);
    m
}

/// Append an MMIE for `key_id` and `ipn` to an unprotected management frame
/// and sign it. `None` for a frame shorter than its header.
pub fn bip_protect(frame: &[u8], key_id: u16, ipn: u64, igtk: &[u8; 16]) -> Option<Vec<u8>> {
    if frame.len() < MAC_HEADER_LEN {
        return None;
    }
    let mut out = Vec::with_capacity(frame.len() + MMIE_LEN);
    out.extend_from_slice(frame);
    out.extend_from_slice(&Mmie { key_id, ipn, mic: [0; 8] }.to_bytes());
    let m = mic(igtk, &out);
    let n = out.len();
    out[n - 8..].copy_from_slice(&m);
    Some(out)
}

/// The frame's MMIE if its MIC verifies under `igtk`. The caller checks the
/// key ID against the IGTK it holds and the IPN against its replay floor. A
/// frame under BIP is not encrypted, so one marked Protected is no BIP
/// frame, and nor is one with the Order bit.
pub fn bip_verify(frame: &[u8], igtk: &[u8; 16]) -> Option<Mmie> {
    let fc = u16::from_le_bytes([*frame.first()?, *frame.get(1)?]);
    if fc & (FC_PROTECTED | FC_ORDER) != 0 {
        return None;
    }
    let mmie = Mmie::from_tail(frame.get(MAC_HEADER_LEN..)?)?;
    let want = mic(igtk, frame);
    // Constant-time compare: the MIC is the secret-dependent value.
    let diff = want.iter().zip(mmie.mic.iter()).fold(0u8, |d, (a, b)| d | (a ^ b));
    (diff == 0).then_some(mmie)
}
