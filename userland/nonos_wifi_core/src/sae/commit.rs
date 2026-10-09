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

use p256::{ProjectivePoint, Scalar};

use super::group::{point_xy, scalar_bytes, scalar_from_bytes, scalar_is_trivial, LEN};
use crate::wpa::kdf::kdf_sha256;
use crate::wpa::sha256::{hmac_sha256, hmac_sha256_parts};

/// One side's commit: the secret rand, the scalar and element it sends, and
/// the PWE they were built from.
#[derive(Clone, Copy)]
pub struct Commit {
    rand: Scalar,
    scalar: Scalar,
    element: ProjectivePoint,
    pwe: ProjectivePoint,
}

/// The keys a completed exchange yields.
#[derive(Clone, Copy)]
pub struct SaeKeys {
    pub kck: [u8; 32],
    pub pmk: [u8; 32],
    pub pmkid: [u8; 16],
}

impl Commit {
    /// Build a commit from the PWE and two 32-byte random values for rand and
    /// mask. `None` if either is not a scalar in (1, r) or their sum is not
    /// (the caller draws again).
    pub fn new(pwe: ProjectivePoint, rand: &[u8; LEN], mask: &[u8; LEN]) -> Option<Commit> {
        let rand = scalar_from_bytes(rand)?;
        let mask = scalar_from_bytes(mask)?;
        if scalar_is_trivial(&rand) || scalar_is_trivial(&mask) {
            return None;
        }
        let scalar = rand + mask;
        if scalar_is_trivial(&scalar) {
            return None;
        }
        let element = -(pwe * mask);
        Some(Commit { rand, scalar, element, pwe })
    }

    /// The commit-scalar as sent.
    pub fn scalar(&self) -> [u8; LEN] {
        scalar_bytes(&self.scalar)
    }

    /// The COMMIT-ELEMENT as sent, x || y. `None` only for the identity, which
    /// a mask in (1, r) times a valid PWE never is.
    pub fn element(&self) -> Option<[u8; 2 * LEN]> {
        point_xy(&self.element)
    }

    /// The keys from the peer's commit. `None` if K is the identity element.
    pub fn derive_keys(&self, peer_scalar: &Scalar, peer_element: &ProjectivePoint) -> Option<SaeKeys> {
        let k_point = (self.pwe * peer_scalar + peer_element) * self.rand;
        let mut kxy = point_xy(&k_point)?;
        let mut keyseed = hmac_sha256(&[0u8; 32], &kxy[..LEN]);
        let context = scalar_bytes(&(self.scalar + peer_scalar));
        let mut both = [0u8; 64];
        let _ = kdf_sha256(&keyseed, b"SAE KCK and PMK", &context, &mut both);
        let mut keys = SaeKeys { kck: [0u8; 32], pmk: [0u8; 32], pmkid: [0u8; 16] };
        keys.kck.copy_from_slice(&both[..32]);
        keys.pmk.copy_from_slice(&both[32..]);
        keys.pmkid.copy_from_slice(&context[..16]);
        super::wipe(&mut kxy);
        super::wipe(&mut keyseed);
        super::wipe(&mut both);
        Some(keys)
    }
}

/// CN(KCK, send-confirm, scalar1, element1, scalar2, element2).
pub fn confirm(
    kck: &[u8; 32],
    send_confirm: u16,
    scalar1: &[u8; LEN],
    element1: &[u8; 2 * LEN],
    scalar2: &[u8; LEN],
    element2: &[u8; 2 * LEN],
) -> [u8; 32] {
    hmac_sha256_parts(kck, &[&send_confirm.to_le_bytes(), scalar1, element1, scalar2, element2])
}
