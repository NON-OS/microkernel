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

//! The password token for `ssid` and `password`, with an optional password
//! identifier.

use super::*;

/// The password token for `ssid` and `password`, with an optional password
/// identifier. Independent of either station's address, so it can be derived
/// once per network.
pub fn derive_pt(
    ssid: &[u8],
    password: &[u8],
    identifier: Option<&[u8]>,
) -> Option<ProjectivePoint> {
    let mut seed = hkdf::extract(ssid, &[password, identifier.unwrap_or(&[])]);
    let mut value = [0u8; PWD_VALUE_LEN];
    let p1 = expand_to_point(&seed, b"SAE Hash to Element u1 P1", &mut value);
    let p2 = expand_to_point(&seed, b"SAE Hash to Element u2 P2", &mut value);
    super::super::wipe(&mut value);
    super::super::wipe(&mut seed);
    Some(p1? + p2?)
}

pub(super) fn expand_to_point(
    seed: &[u8; 32],
    label: &[u8],
    value: &mut [u8; PWD_VALUE_LEN],
) -> Option<ProjectivePoint> {
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
