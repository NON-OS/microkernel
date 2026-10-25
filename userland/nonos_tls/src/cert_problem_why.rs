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

//! The two findings cert_problem.rs words: a window the clock is outside
//! of, and a chain no trusted root anchors.

use super::cert_problem::CertProblem;

pub fn window_problem(cert: &[u8], now: u64) -> Option<CertProblem> {
    let Some((from, until)) = super::cert_window::cert_window(cert) else {
        return Some(CertProblem::Unreadable);
    };
    if now < from {
        Some(CertProblem::NotYetValid)
    } else if now > until {
        Some(CertProblem::Expired)
    } else {
        None
    }
}

/*
 * A `devnet` build's throwaway root anchors only through its own gate, which
 * checks the leaf's signature: that root's key is public, so a name match
 * alone would report a leaf anyone signed as trusted. Without the feature the
 * gate answers no, as the walk's does.
 */
pub fn anchored(top: &[u8], leaf: &[u8], host: &[u8], len: u8) -> bool {
    let pinned = super::cert_spki::cert_spki(top)
        .and_then(super::hash_sha256::hash_sha256)
        .is_some_and(|h| super::roots::is_trusted_spki_hash(&h));
    pinned
        || super::cert_issuer::cert_issuer(top)
            .and_then(super::roots::find_spki_by_subject)
            .is_some()
        || super::roots::dev_anchors(top, leaf, host, len) == Ok(true)
}
