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

//! A scalar from 32 bytes, if it is below the order.

use super::*;

/// A scalar from 32 bytes, if it is below the order.
pub fn scalar_from_bytes(b: &[u8; LEN]) -> Option<Scalar> {
    Option::from(Scalar::from_repr(FieldBytes::from(*b)))
}

/// A 32-byte value reduced modulo the order.
pub fn scalar_reduce(b: &[u8; LEN]) -> Scalar {
    <Scalar as Reduce<U256>>::reduce_bytes(&FieldBytes::from(*b))
}

/// A scalar's 32-byte big-endian encoding.
pub fn scalar_bytes(s: &Scalar) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    out.copy_from_slice(&s.to_bytes());
    out
}

/// Whether a scalar is 0 or 1, which SAE never accepts (1 < scalar < r).
pub fn scalar_is_trivial(s: &Scalar) -> bool {
    bool::from(s.is_zero()) || *s == Scalar::ONE
}

/// The point (x, y), if both coordinates are below p and it lies on the curve.
pub fn point_from_xy(x: &[u8; LEN], y: &[u8; LEN]) -> Option<ProjectivePoint> {
    let ep =
        EncodedPoint::from_affine_coordinates(&FieldBytes::from(*x), &FieldBytes::from(*y), false);
    let a: Option<AffinePoint> = Option::from(AffinePoint::from_encoded_point(&ep));
    a.map(ProjectivePoint::from)
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
