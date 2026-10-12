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

//! This host's key pair for one pairing. The secret comes from the
//! caller's random source and is used for one pairing only, as Vol 3,
//! Part H, 2.3.5.6.1 asks; it is overwritten when the pair is dropped.

use super::peer::PeerKey;
use crate::error::SmpError;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::elliptic_curve::{Field, PrimeField};
use p256::{FieldBytes, ProjectivePoint, Scalar};

pub struct KeyPair {
    secret: Scalar,
    x: [u8; 32],
    y: [u8; 32],
}

impl KeyPair {
    /// The pair for a 32-octet secret, most significant octet first;
    /// `BadSecret` if it is zero or not below the group order, so the
    /// caller draws again.
    pub fn from_secret(secret: [u8; 32]) -> Result<Self, SmpError> {
        let s = Option::<Scalar>::from(Scalar::from_repr(FieldBytes::from(secret)));
        let s = s.filter(|s| !bool::from(s.is_zero())).ok_or(SmpError::BadSecret)?;
        let point = (ProjectivePoint::GENERATOR * s).to_affine().to_encoded_point(false);
        let (Some(x), Some(y)) = (point.x(), point.y()) else { return Err(SmpError::BadSecret) };
        Ok(Self { secret: s, x: (*x).into(), y: (*y).into() })
    }

    /// The public key's coordinates, most significant octet first.
    pub fn public(&self) -> (&[u8; 32], &[u8; 32]) {
        (&self.x, &self.y)
    }

    /// The DHKey: the x coordinate of the secret times the peer's point,
    /// most significant octet first (Vol 3, Part H, 2.3.5.6.1).
    pub fn dh_key(&self, peer: &PeerKey) -> Result<[u8; 32], SmpError> {
        let shared = (ProjectivePoint::from(*peer.point()) * self.secret).to_affine();
        let encoded = shared.to_encoded_point(false);
        encoded.x().map(|x| (*x).into()).ok_or(SmpError::BadKey)
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        // SAFETY: `secret` is a valid, aligned field of `self`.
        unsafe { core::ptr::write_volatile(&mut self.secret, Scalar::ZERO) };
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
