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

//! Build a commit from the PWE and two 32-byte random values for rand and
//! mask.

use super::*;

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
    pub fn derive_keys(
        &self,
        peer_scalar: &Scalar,
        peer_element: &ProjectivePoint,
    ) -> Option<SaeKeys> {
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
        super::super::wipe(&mut kxy);
        super::super::wipe(&mut keyseed);
        super::super::wipe(&mut both);
        Some(keys)
    }
}
