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

//! Points to and from their x || y encoding. A point is taken only if both
//! coordinates are below p and it lies on the curve: a point off it lets
//! whoever chose it learn a secret scalar a few bits at a time (the
//! invalid-curve attack), so it is refused before any arithmetic.

use crate::constants::LEN;
use p256::elliptic_curve::group::Group;
use p256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use p256::{AffinePoint, EncodedPoint, FieldBytes, ProjectivePoint};

/// The point (x, y), if both coordinates are below p and it lies on the
/// curve. The identity has no affine coordinates, so it is never returned.
pub fn point_from_xy(x: &[u8; LEN], y: &[u8; LEN]) -> Option<ProjectivePoint> {
    let ep =
        EncodedPoint::from_affine_coordinates(&FieldBytes::from(*x), &FieldBytes::from(*y), false);
    let a: Option<AffinePoint> = Option::from(AffinePoint::from_encoded_point(&ep));
    a.map(ProjectivePoint::from).filter(|p| !bool::from(p.is_identity()))
}

/// A point's x || y encoding, or `None` for the identity, which has none.
pub fn point_xy(p: &ProjectivePoint) -> Option<[u8; 2 * LEN]> {
    if bool::from(p.is_identity()) {
        return None;
    }
    let ep = p.to_affine().to_encoded_point(false);
    let (x, y) = (ep.x()?, ep.y()?);
    let mut out = [0u8; 2 * LEN];
    out[..LEN].copy_from_slice(x);
    out[LEN..].copy_from_slice(y);
    Some(out)
}
