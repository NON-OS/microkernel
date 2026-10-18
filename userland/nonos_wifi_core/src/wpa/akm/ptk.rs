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

//! The pairwise transient key each AKM derives: PRF-SHA1 for PSK, KDF-SHA256
//! for PSK-SHA256 and SAE.

use super::*;

impl Akm {
    /// The pairwise transient key of a CCMP-128 link (KCK || KEK || TK, 16
    /// octets each).
    pub fn derive_ptk(
        self,
        pmk: &[u8; 32],
        aa: &[u8; 6],
        spa: &[u8; 6],
        anonce: &[u8; 32],
        snonce: &[u8; 32],
    ) -> [u8; 48] {
        let mut out = [0u8; 48];
        self.derive_ptk_into(pmk, (aa, spa), (anonce, snonce), &mut out);
        out
    }

    /// The pairwise transient key at `out`'s length, which the pairwise
    /// cipher sets (48 octets for CCMP-128, 64 for GCMP-256). False only
    /// for a length no PTK has.
    pub fn derive_ptk_into(
        self,
        pmk: &[u8; 32],
        (aa, spa): (&[u8; 6], &[u8; 6]),
        (anonce, snonce): (&[u8; 32], &[u8; 32]),
        out: &mut [u8],
    ) -> bool {
        match self {
            Akm::Psk => {
                ptk_into(pmk, aa, spa, anonce, snonce, out);
                true
            }
            Akm::PskSha256 | Akm::Sae => ptk_sha256_into(pmk, aa, spa, anonce, snonce, out),
        }
    }
}
