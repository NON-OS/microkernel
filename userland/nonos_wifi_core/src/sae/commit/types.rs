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

//! The SAE commit and the keys it leads to (IEEE Std 802.11-2020, 12.4.5).
//!
//!   commit-scalar  = (rand + mask) mod r
//!   COMMIT-ELEMENT = inverse(mask * PWE)
//!   K  = rand * (peer-commit-scalar * PWE + PEER-COMMIT-ELEMENT), never the identity
//!   keyseed = HMAC-SHA256(0^32, x(K))
//!   KCK || PMK = KDF-SHA256-512(keyseed, "SAE KCK and PMK",
//!                               (commit-scalar + peer-commit-scalar) mod r)
//!   PMKID = the first 16 octets of that sum
//!   confirm = HMAC-SHA256(KCK, send-confirm || commit-scalar || COMMIT-ELEMENT
//!                              || peer-commit-scalar || PEER-COMMIT-ELEMENT)
//!
//! The salt is all zero for hunting and pecking and for hash-to-element with no
//! rejected groups, which is the only H2E case this station produces (it
//! offers group 19 alone). Checked against the Annex J.10 commit, KCK, PMK and
//! PMKID vector.

use super::*;

/// One side's commit: the secret rand, the scalar and element it sends, and
/// the PWE they were built from.
pub struct Commit {
    pub(super) rand: Scalar,
    pub(super) scalar: Scalar,
    pub(super) element: ProjectivePoint,
    pub(super) pwe: ProjectivePoint,
}

/// The keys a completed exchange yields.
pub struct SaeKeys {
    pub kck: [u8; 32],
    pub pmk: [u8; 32],
    pub pmkid: [u8; 16],
}
