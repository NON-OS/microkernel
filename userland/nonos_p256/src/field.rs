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

//! Field elements: the curve's coefficients, its right-hand side, and
//! conversions from octets that never branch on the value.

use crate::constants::{fe_const, CURVE_B, LEN, PRIME, TWO_256_MOD_P};
use p256::elliptic_curve::bigint::Encoding;
use p256::elliptic_curve::subtle::{Choice, ConditionallySelectable};
use p256::{FieldBytes, FieldElement, U256};

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
