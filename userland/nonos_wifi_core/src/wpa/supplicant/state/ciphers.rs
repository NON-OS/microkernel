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

//! The ciphers a handshake keys, read from the station's own RSNE: it names
//! the group cipher and the one pairwise cipher negotiation chose, and
//! message 2 repeats it, so the keys cannot disagree with what the station
//! told the access point. An element that will not parse means CCMP-128,
//! the cipher of the fixed WPA2 element `new` sends.

use super::*;

pub(super) fn own_ciphers(own_rsne: &[u8]) -> (Cipher, Cipher) {
    let r = own_rsne.get(2..).and_then(parse_rsne);
    let group = r.and_then(|r| Cipher::from_selector(r.group)).unwrap_or(Cipher::Ccmp128);
    let gcmp = r.is_some_and(|r| r.pairwise_gcmp256);
    (if gcmp { Cipher::Gcmp256 } else { Cipher::Ccmp128 }, group)
}

impl Supplicant {
    /// The pairwise cipher the PTK's TK is for.
    pub fn pairwise_cipher(&self) -> Cipher {
        self.pairwise
    }

    /// The group cipher the GTK is for.
    pub fn group_cipher(&self) -> Cipher {
        self.group
    }
}
