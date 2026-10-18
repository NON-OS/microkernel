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

//! SAE hash-to-element (IEEE Std 802.11-2020, 12.4.4.2.3): the password
//! element derived without a loop, so its timing says nothing about the
//! password. The password token PT depends only on the SSID and the password
//! (and an optional password identifier):
//!
//!   pwd-seed = HKDF-Extract(ssid, password [|| identifier])
//!   u1 = HKDF-Expand(pwd-seed, "SAE Hash to Element u1 P1", 48) mod p
//!   u2 = HKDF-Expand(pwd-seed, "SAE Hash to Element u2 P2", 48) mod p
//!   PT = SSWU(u1) + SSWU(u2)
//!
//! and the PWE for one pair of stations is val * PT, with
//! val = HMAC-SHA256(0^32, MAX(MAC) || MIN(MAC)) mod (r - 1) + 1. SSWU is the
//! simplified Shallue-van de Woestijne-Ulas map with z = -10 for group 19,
//! written as the standard writes it, every selection a constant-time select.
//! Checked against the IEEE Std 802.11-2020 Annex J.10 PT/PWE vector.

use super::*;

/// olen(p) + ceil(olen(p)/2): the bytes expanded per u, so u mod p is close
/// to uniform.
pub(super) const PWD_VALUE_LEN: usize = LEN + LEN.div_ceil(2);

/// The SSWU map of `u` onto P-256 (z = -10). `None` only if the map produced
/// no square root, which the construction rules out.
pub fn sswu(u: &FieldElement) -> Option<ProjectivePoint> {
    let a = curve_a();
    let b = curve_b();
    let z = -FieldElement::from_u64(10);
    let u2 = u.square();
    // m = z^2 * u^4 + z * u^2, computed as t1^2 + t1 with t1 = z * u^2.
    let t1 = z * u2;
    let m = t1.square() + t1;
    let m_is_zero = m.is_zero();
    // t = m^(p-2), which is zero for m = 0.
    let t = m.invert().unwrap_or(FieldElement::ZERO);
    // The two candidates for x1: b / (z * a) when m is zero, else
    // (-b / a) * (1 + t). a and z * a are nonzero constants.
    let x1a = b * (z * a).invert().unwrap_or(FieldElement::ZERO);
    let x1b = -b * a.invert().unwrap_or(FieldElement::ZERO) * (FieldElement::ONE + t);
    let x1 = FieldElement::conditional_select(&x1b, &x1a, m_is_zero);
    let gx1 = curve_rhs(&x1);
    let x2 = t1 * x1;
    let gx2 = curve_rhs(&x2);
    // l = gx1 is a quadratic residue (zero counts as one).
    let gx1_is_square = gx1.sqrt().is_some();
    let v = FieldElement::conditional_select(&gx2, &gx1, gx1_is_square);
    let x = FieldElement::conditional_select(&x2, &x1, gx1_is_square);
    let y: FieldElement = Option::from(v.sqrt())?;
    // P = (x, y) when LSB(u) = LSB(y), else (x, p - y).
    let same_parity = !(u.is_odd() ^ y.is_odd());
    let y = FieldElement::conditional_select(&-y, &y, same_parity);
    point_from_xy(&fe_bytes(&x), &fe_bytes(&y))
}
