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

//! Elliptic-curve Diffie-Hellman with one secret per exchange. The secret
//! comes from the caller's random source; it is overwritten when dropped,
//! so it does not outlive the exchange it was drawn for.

use crate::constants::LEN;
use crate::point::point_xy;
use p256::elliptic_curve::Field;
use p256::{ProjectivePoint, Scalar};

pub struct Secret(Scalar);

impl Secret {
    /// The secret for 32 octets, most significant first, if it lies in
    /// [1, r - 1]. Anything else is refused rather than reduced, so a
    /// caller with a biased or broken source draws again.
    pub fn from_bytes(b: &[u8; LEN]) -> Option<Self> {
        crate::scalar::scalar_from_bytes(b).filter(|s| !bool::from(s.is_zero())).map(Self)
    }

    /// The public key, x || y: the secret times the generator.
    pub fn public_xy(&self) -> Option<[u8; 2 * LEN]> {
        point_xy(&(ProjectivePoint::GENERATOR * self.0))
    }

    /// The shared secret: the x coordinate of the secret times `peer`, a
    /// point already checked by `point_from_xy`. `None` only if the product
    /// is the identity, which a valid peer point cannot give for a secret
    /// below r.
    pub fn shared_x(&self, peer: &ProjectivePoint) -> Option<[u8; LEN]> {
        let xy = point_xy(&(*peer * self.0))?;
        let mut x = [0u8; LEN];
        x.copy_from_slice(&xy[..LEN]);
        Some(x)
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        // SAFETY: `self.0` is a valid, aligned field of `self`.
        unsafe { core::ptr::write_volatile(&mut self.0, Scalar::ZERO) };
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
