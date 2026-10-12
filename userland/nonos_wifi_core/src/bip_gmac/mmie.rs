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

//! The Management MIC element (IEEE Std 802.11-2020, 9.4.2.54) as
//! BIP-GMAC-256 carries it: element id 76, length 24, the key id (2 octets)
//! and the IPN (6 octets), both least significant octet first, then the
//! 16-octet MIC. It is the last element of the frame body.

pub const ELEMENT_ID: u8 = 76;
pub const MIC_LEN: usize = 16;
pub const MMIE_LEN: usize = 2 + 2 + 6 + MIC_LEN;
pub const IPN_MAX: u64 = (1 << 48) - 1;

/// Where the MIC starts inside the element.
pub(super) const MIC_AT: usize = MMIE_LEN - MIC_LEN;

pub(super) struct Mmie {
    pub key_id: u16,
    pub ipn: u64,
    pub mic: [u8; MIC_LEN],
}

/// The element with its MIC field zero, as the MIC is computed over it.
pub(super) fn zeroed(key_id: u16, ipn: u64) -> [u8; MMIE_LEN] {
    let mut e = [0u8; MMIE_LEN];
    e[0] = ELEMENT_ID;
    e[1] = (MMIE_LEN - 2) as u8;
    e[2..4].copy_from_slice(&key_id.to_le_bytes());
    e[4..10].copy_from_slice(&ipn.to_le_bytes()[..6]);
    e
}

/// The element at the end of `body`, if it is one of this length.
pub(super) fn read(body: &[u8]) -> Option<Mmie> {
    let e = body.get(body.len().checked_sub(MMIE_LEN)?..)?;
    if e[0] != ELEMENT_ID || usize::from(e[1]) != MMIE_LEN - 2 {
        return None;
    }
    let mut ipn = [0u8; 8];
    ipn[..6].copy_from_slice(&e[4..10]);
    Some(Mmie {
        key_id: u16::from_le_bytes([e[2], e[3]]),
        ipn: u64::from_le_bytes(ipn),
        mic: e[MIC_AT..].try_into().ok()?,
    })
}
