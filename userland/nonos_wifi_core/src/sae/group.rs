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

//! IEEE 802.11 group 19, the NIST P-256 curve y^2 = x^3 - 3x + b over GF(p),
//! as SAE uses it: field elements and scalars to and from their 32-byte
//! big-endian octet strings, and points to and from their x || y encoding.
//! The arithmetic is the p256 crate's (constant time); this file only adds the
//! conversions SAE needs and refuses anything that is not a valid element.

use p256::elliptic_curve::bigint::Encoding;
use p256::elliptic_curve::group::Group;
use p256::elliptic_curve::ops::Reduce;
use p256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use p256::elliptic_curve::subtle::{Choice, ConditionallySelectable};
use p256::elliptic_curve::{Field, PrimeField};
use p256::{AffinePoint, EncodedPoint, FieldBytes, FieldElement, ProjectivePoint, Scalar, U256};

/// The IEEE 802.11 number of this group.
pub const GROUP_19: u16 = 19;
/// The length of a field element, a scalar, and each point coordinate.
pub const LEN: usize = 32;

/// The prime p, big-endian.
pub const PRIME: [u8; LEN] = hex32("ffffffff00000001000000000000000000000000ffffffffffffffffffffffff");
/// The group order r, big-endian.
pub const ORDER: [u8; LEN] = hex32("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
/// The curve coefficient b, big-endian (a is -3).
pub const CURVE_B: [u8; LEN] = hex32("5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b");
/// 2^256 mod p, to reduce a value longer than one field element.
pub const TWO_256_MOD_P: [u8; LEN] =
    hex32("00000000fffffffeffffffffffffffffffffffff000000000000000000000001");

// A big-endian 32-byte constant from 64 hex digits. Only ever evaluated for the
// constants above, at compile time, so a bad digit stops the build.
const fn hex32(s: &str) -> [u8; LEN] {
    let b = s.as_bytes();
    let mut out = [0u8; LEN];
    let mut i = 0;
    while i < LEN {
        out[i] = (hex_digit(b[2 * i]) << 4) | hex_digit(b[2 * i + 1]);
        i += 1;
    }
    out
}

/// A lowercase hex digit's value. Only the constants above are decoded with
/// it, and the Annex J.10 known answers in `nonos_wifi_core_proofs` fail if
/// one of them is mistyped, so any other character reads as zero.
const fn hex_digit(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => 0,
    }
}

/// The field element of a constant known to be below p.
pub fn fe_const(b: &[u8; LEN]) -> FieldElement {
    FieldElement::from_bytes(&FieldBytes::from(*b)).unwrap_or(FieldElement::ZERO)
}

/// The curve's b, and a = -3.
pub fn curve_b() -> FieldElement {
    fe_const(&CURVE_B)
}

pub fn curve_a() -> FieldElement {
    -FieldElement::from_u64(3)
}

/// x^3 + a*x + b.
pub fn curve_rhs(x: &FieldElement) -> FieldElement {
    x.square() * x + curve_a() * x + curve_b()
}

/// A 32-byte string as a field element when it is below p, with the answer as
/// a `Choice` so a secret value is converted without branching on it. A value
/// at or above p yields zero and a false choice.
pub fn fe_from_bytes_ct(b: &[u8; LEN]) -> (FieldElement, Choice) {
    let r = FieldElement::from_bytes(&FieldBytes::from(*b));
    (r.unwrap_or(FieldElement::ZERO), r.is_some())
}

/// A 32-byte value reduced modulo p, constant time. Any 256-bit value is below
/// 2p, so one conditional subtraction reduces it.
pub fn fe_reduce32(b: &[u8; LEN]) -> FieldElement {
    let (direct, below) = fe_from_bytes_ct(b);
    let minus_p = U256::from_be_slice(b).wrapping_sub(&U256::from_be_slice(&PRIME));
    let (reduced, _) = fe_from_bytes_ct(&minus_p.to_be_bytes());
    FieldElement::conditional_select(&reduced, &direct, below)
}

/// A big-endian value of up to 64 bytes reduced modulo p, constant time in the
/// value: split into a high and a low 256-bit half, hi * 2^256 + lo.
pub fn fe_reduce_wide(v: &[u8]) -> Option<FieldElement> {
    if v.len() > 2 * LEN {
        return None;
    }
    let mut hi = [0u8; LEN];
    let mut lo = [0u8; LEN];
    let split = v.len().saturating_sub(LEN);
    hi[LEN - split..].copy_from_slice(&v[..split]);
    lo[LEN - (v.len() - split)..].copy_from_slice(&v[split..]);
    Some(fe_reduce32(&hi) * fe_const(&TWO_256_MOD_P) + fe_reduce32(&lo))
}

/// A field element's 32-byte big-endian encoding.
pub fn fe_bytes(f: &FieldElement) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    out.copy_from_slice(&f.to_bytes());
    out
}

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
    let ep = EncodedPoint::from_affine_coordinates(&FieldBytes::from(*x), &FieldBytes::from(*y), false);
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
