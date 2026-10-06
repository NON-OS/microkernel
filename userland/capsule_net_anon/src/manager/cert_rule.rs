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

//! Which authority certificates the client still has to fetch.
//!
//! The first sweep over the authorities used to end the moment one
//! certificate anchored. A boot whose lease landed halfway through a sweep
//! held three, the quorum needs four, and every consensus after that failed
//! with "held certs and needed 3 4": the missing certificates were never
//! asked for again. Certificates are now kept across sweeps, only the ones
//! not held are fetched, and the sweep is done only when a quorum's worth is
//! in. A consensus that names a signing key the client does not hold for an
//! authority (one never fetched, or one rotated since) sends the client back
//! for that authority's certificate. The one held is kept until a newer one
//! anchors: the directory transport is not authenticated, so a consensus
//! alone is not reason enough to give up a certificate that anchored.
//!
//! Pure. Held in anon_ntor_proofs.

extern crate alloc;

use alloc::vec::Vec;

/// Whether enough certificates are held for a consensus to reach its
/// quorum at all. Fewer, and fetching a consensus can only fail.
pub fn enough(held: usize, required: usize) -> bool {
    held >= required
}

/// Whether the sweep has to fetch authority `index`: it does unless a
/// certificate for it is held, or when the last consensus named a key for
/// it other than the one held (`refetch`).
pub fn wanted(index: usize, held: &[usize], refetch: &[usize]) -> bool {
    !held.contains(&index) || refetch.contains(&index)
}

/// The authorities a consensus's signatures name whose named signing key is
/// not the one the client holds for them, in authority order and each once.
///
/// `named` is each signature's (identity, signing key digest); `held` is
/// each held certificate's (authority index, signing key digest); and
/// `authorities` the v3 identities, by index. A line naming no known
/// authority is skipped: it could never count towards the quorum.
pub fn unheld(
    named: &[([u8; 20], [u8; 20])],
    held: &[(usize, [u8; 20])],
    authorities: &[[u8; 20]],
) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for (identity, signing) in named {
        let Some(index) = authorities.iter().position(|known| known == identity) else {
            continue;
        };
        let matches = held.iter().any(|(at, digest)| *at == index && digest == signing);
        if !matches && !out.contains(&index) {
            out.push(index);
        }
    }
    out.sort_unstable();
    out
}
