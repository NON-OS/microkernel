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

//! The Management MIC element (IEEE Std 802.11-2020, 9.4.2.54): the last
//! element of a group-addressed robust management frame under BIP, carrying
//! the IGTK's index, the frame's IGTK packet number (IPN) and its MIC.

pub const EID_MMIE: u8 = 76;
/// The whole element for BIP-CMAC-128: ID, length 16, key ID, IPN, MIC.
pub const MMIE_LEN: usize = 18;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mmie {
    pub key_id: u16,
    /// Little-endian on the air, as in the IGTK KDE.
    pub ipn: u64,
    pub mic: [u8; 8],
}

impl Mmie {
    /// The MMIE that ends `body`, a management frame body. It must be the
    /// last element, so it is read from the tail and nowhere else.
    pub fn from_tail(body: &[u8]) -> Option<Self> {
        let at = body.len().checked_sub(MMIE_LEN)?;
        let e = &body[at..];
        if e[0] != EID_MMIE || e[1] as usize != MMIE_LEN - 2 {
            return None;
        }
        let mut ipn = [0u8; 8];
        ipn[..6].copy_from_slice(&e[4..10]);
        let mut mic = [0u8; 8];
        mic.copy_from_slice(&e[10..18]);
        Some(Self { key_id: u16::from_le_bytes([e[2], e[3]]), ipn: u64::from_le_bytes(ipn), mic })
    }

    pub fn to_bytes(&self) -> [u8; MMIE_LEN] {
        let mut e = [0u8; MMIE_LEN];
        e[0] = EID_MMIE;
        e[1] = (MMIE_LEN - 2) as u8;
        e[2..4].copy_from_slice(&self.key_id.to_le_bytes());
        e[4..10].copy_from_slice(&self.ipn.to_le_bytes()[..6]);
        e[10..18].copy_from_slice(&self.mic);
        e
    }
}
