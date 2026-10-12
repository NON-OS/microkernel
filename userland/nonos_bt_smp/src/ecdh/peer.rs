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

//! The peer's public key, accepted only if it is a point on P-256 other
//! than the point at infinity. A key off the curve lets a peer learn the
//! secret a few bits at a time (the invalid-curve attack the 2018 erratum
//! to Vol 3, Part H, 2.3.5.6.1 closed), so it is refused before any
//! arithmetic; so is the published debug key.

use super::debug::{DEBUG_PUBLIC_X, DEBUG_PUBLIC_Y};
use crate::error::SmpError;
use p256::elliptic_curve::sec1::FromEncodedPoint;
use p256::{AffinePoint, EncodedPoint, FieldBytes};

pub struct PeerKey {
    point: AffinePoint,
    x: [u8; 32],
}

impl PeerKey {
    /// From coordinates most significant octet first.
    pub fn new(x: &[u8; 32], y: &[u8; 32]) -> Result<Self, SmpError> {
        if *x == DEBUG_PUBLIC_X && *y == DEBUG_PUBLIC_Y {
            return Err(SmpError::DebugKey);
        }
        let (fx, fy) = (FieldBytes::from(*x), FieldBytes::from(*y));
        let encoded = EncodedPoint::from_affine_coordinates(&fx, &fy, false);
        let point = Option::<AffinePoint>::from(AffinePoint::from_encoded_point(&encoded));
        let point = point.filter(|p| !bool::from(p.is_identity())).ok_or(SmpError::BadKey)?;
        Ok(Self { point, x: *x })
    }

    pub(super) fn point(&self) -> &AffinePoint {
        &self.point
    }

    /// The x coordinate, most significant octet first, as f4 and g2 take it.
    pub fn x(&self) -> &[u8; 32] {
        &self.x
    }
}
