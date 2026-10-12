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

//! BIP-CMAC-128 (IEEE Std 802.11-2020, 12.5.4): how a group-addressed robust
//! management frame, a broadcast deauthentication above all, is protected.
//! It is not encrypted. Its last element, the MME, carries the IGTK's index,
//! a packet number (IPN) and a MIC: AES-128-CMAC under the IGTK, cut to 64
//! bits, over the header's frame control (Retry, Power Management and More
//! Data cleared) and three addresses, then the body with the MIC zeroed.

use super::header::{FC_PROTECTED, MAC_HEADER_LEN};
use crate::ccmp::cmac::aes_cmac_parts;

/// The Management MIC element.
pub const ELEMENT_MME: u8 = 76;
/// The MME's whole length: id, length, key index (2), IPN (6), MIC (8).
pub const MME_LEN: usize = 18;
const MIC_LEN: usize = 8;
/// Retry, Power Management and More Data, masked out of the AAD.
const FC_MUTABLE: u16 = 0x3800;
/// The Order bit: a management frame with it set carries an HT Control field
/// after Address 3, which this check does not take.
const FC_ORDER: u16 = 0x8000;

/// The key index and packet number of a frame whose MIC held.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mme {
    pub key_id: u16,
    pub ipn: u64,
}

/// The MME of `frame` (a whole management frame) if it ends in one and its
/// MIC holds under `igtk`. The caller checks the key index and the IPN.
pub fn bip_open(frame: &[u8], igtk: &[u8; 16]) -> Option<Mme> {
    let fc = u16::from_le_bytes([*frame.first()?, *frame.get(1)?]);
    if frame.len() < MAC_HEADER_LEN + MME_LEN || fc & (FC_PROTECTED | FC_ORDER) != 0 {
        return None;
    }
    let mme = &frame[frame.len() - MME_LEN..];
    if mme[0] != ELEMENT_MME || usize::from(mme[1]) != MME_LEN - 2 {
        return None;
    }
    let mut aad = [0u8; 20];
    aad[..2].copy_from_slice(&(fc & !FC_MUTABLE).to_le_bytes());
    aad[2..].copy_from_slice(&frame[4..22]);
    let signed = &frame[MAC_HEADER_LEN..frame.len() - MIC_LEN];
    let mac = aes_cmac_parts(igtk, &[&aad, signed, &[0u8; MIC_LEN]]);
    let mut diff = 0u8;
    for (a, b) in mac[..MIC_LEN].iter().zip(&mme[10..]) {
        diff |= a ^ b;
    }
    if diff != 0 {
        return None;
    }
    let mut ipn = [0u8; 8];
    ipn[..6].copy_from_slice(&mme[4..10]);
    Some(Mme { key_id: u16::from_le_bytes([mme[2], mme[3]]), ipn: u64::from_le_bytes(ipn) })
}
