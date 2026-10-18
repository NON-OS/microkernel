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

//! The pairwise transient key at the length the link needs: the KCK and KEK
//! (16 octets each for these AKMs) then the pairwise cipher's TK, so 48
//! octets under CCMP-128 and 64 under GCMP-256 (IEEE Std 802.11-2020,
//! 12.7.1.3). The length is the output buffer's; the PRF and the KDF both
//! take it from there, and the KDF also feeds it into the derivation.

use super::kdf::kdf_sha256;
use super::prf::prf;

const LABEL: &[u8] = b"Pairwise key expansion";

/// PRF-SHA1 (the PSK AKM) over the ordered addresses and nonces, filling
/// `out`. Both sides order them smaller first, so they agree whichever of
/// them is the authenticator.
pub fn ptk_into(
    pmk: &[u8],
    aa: &[u8; 6],
    spa: &[u8; 6],
    anonce: &[u8; 32],
    snonce: &[u8; 32],
    out: &mut [u8],
) {
    prf(pmk, LABEL, &ordered_context(aa, spa, anonce, snonce), out);
}

/// KDF-SHA256 (SAE, PSK-SHA256) over the same context, filling `out`.
/// False only for an output past the KDF's 16-bit length in bits, which no
/// PTK comes near.
pub fn ptk_sha256_into(
    pmk: &[u8],
    aa: &[u8; 6],
    spa: &[u8; 6],
    anonce: &[u8; 32],
    snonce: &[u8; 32],
    out: &mut [u8],
) -> bool {
    kdf_sha256(pmk, LABEL, &ordered_context(aa, spa, anonce, snonce), out)
}

// Min(AA,SPA) || Max(AA,SPA) || Min(ANonce,SNonce) || Max(ANonce,SNonce).
fn ordered_context(aa: &[u8; 6], spa: &[u8; 6], anonce: &[u8; 32], snonce: &[u8; 32]) -> [u8; 76] {
    let (a1, a2) = if aa <= spa { (aa, spa) } else { (spa, aa) };
    let (n1, n2) = if anonce <= snonce { (anonce, snonce) } else { (snonce, anonce) };
    let mut data = [0u8; 76];
    data[0..6].copy_from_slice(a1);
    data[6..12].copy_from_slice(a2);
    data[12..44].copy_from_slice(n1);
    data[44..76].copy_from_slice(n2);
    data
}
