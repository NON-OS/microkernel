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

use p256::elliptic_curve::bigint::Encoding;
use p256::elliptic_curve::subtle::ConditionallySelectable;
use p256::{FieldElement, ProjectivePoint, U256};

use super::group::{
    curve_a, curve_b, curve_rhs, fe_bytes, fe_reduce_wide, point_from_xy, scalar_from_bytes, LEN,
    ORDER,
};
use crate::wpa::hkdf;
use crate::wpa::sha256::hmac_sha256_parts;

/// olen(p) + ceil(olen(p)/2): the bytes expanded per u, so u mod p is close
/// to uniform.
const PWD_VALUE_LEN: usize = LEN + LEN.div_ceil(2);

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

/// The password token for `ssid` and `password`, with an optional password
/// identifier. Independent of either station's address, so it can be derived
/// once per network.
pub fn derive_pt(ssid: &[u8], password: &[u8], identifier: Option<&[u8]>) -> Option<ProjectivePoint> {
    let mut seed = hkdf::extract(ssid, &[password, identifier.unwrap_or(&[])]);
    let mut value = [0u8; PWD_VALUE_LEN];
    let p1 = expand_to_point(&seed, b"SAE Hash to Element u1 P1", &mut value);
    let p2 = expand_to_point(&seed, b"SAE Hash to Element u2 P2", &mut value);
    super::wipe(&mut value);
    super::wipe(&mut seed);
    Some(p1? + p2?)
}

fn expand_to_point(seed: &[u8; 32], label: &[u8], value: &mut [u8; PWD_VALUE_LEN]) -> Option<ProjectivePoint> {
    if !hkdf::expand(seed, label, value) {
        return None;
    }
    sswu(&fe_reduce_wide(value)?)
}

/// The password element for two stations from the password token.
pub fn pwe_from_pt(pt: &ProjectivePoint, a: &[u8; 6], b: &[u8; 6]) -> Option<ProjectivePoint> {
    let (hi, lo) = if a > b { (a, b) } else { (b, a) };
    let val = hmac_sha256_parts(&[0u8; 32], &[hi, lo]);
    // val mod (r - 1) + 1: any 256-bit value is below 2(r - 1), so one
    // conditional subtraction reduces it. The addresses are public, so this
    // needs no constant-time care.
    let r_minus_1 = U256::from_be_slice(&ORDER).wrapping_sub(&U256::ONE);
    let mut v = U256::from_be_slice(&val);
    if v >= r_minus_1 {
        v = v.wrapping_sub(&r_minus_1);
    }
    let s = scalar_from_bytes(&v.wrapping_add(&U256::ONE).to_be_bytes())?;
    Some(*pt * s)
}
