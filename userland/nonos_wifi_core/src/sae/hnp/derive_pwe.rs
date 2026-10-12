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

//! The password element for two stations, or `None` if `decoy` is not as
//! long as `password`, the password is too long, or (beyond any practical
//! chance) no element was found.

use super::*;

/// The password element for two stations, or `None` if `decoy` is not as
/// long as `password`, the password is too long, or (beyond any practical
/// chance) no element was found. `decoy` must be fresh random bytes: it is
/// hashed in the password's place once the element is found.
pub fn derive_pwe(
    password: &[u8],
    decoy: &[u8],
    a: &[u8; 6],
    b: &[u8; 6],
) -> Option<ProjectivePoint> {
    if password.len() > PASSWORD_MAX || decoy.len() != password.len() {
        return None;
    }
    let mut key = [0u8; 12];
    let (hi, lo) = if a > b { (a, b) } else { (b, a) };
    key[..6].copy_from_slice(hi);
    key[6..].copy_from_slice(lo);

    let mut work = [0u8; PASSWORD_MAX];
    let mut value = [0u8; LEN];
    let mut found = Choice::from(0);
    let mut x_sel = FieldElement::ZERO;
    let mut odd_sel = 0u8;
    let mut counter: u8 = 1;
    while counter <= MAX_ROUNDS {
        if counter > MIN_ROUNDS && bool::from(found) {
            break;
        }
        // Hash the real password until the element is found, the stand-in after.
        for (w, (p, s)) in work.iter_mut().zip(password.iter().zip(decoy.iter())) {
            *w = u8::conditional_select(p, s, found);
        }
        let mut seed = hmac_sha256_parts(&key, &[&work[..password.len()], &[counter]]);
        let _ = kdf_sha256(&seed, b"SAE Hunting and Pecking", &PRIME, &mut value);
        let (x, below_p) = fe_from_bytes_ct(&value);
        let on_curve = curve_rhs(&x).sqrt().is_some();
        let take = below_p & on_curve & !found;
        x_sel = FieldElement::conditional_select(&x_sel, &x, take);
        odd_sel = u8::conditional_select(&odd_sel, &(seed[LEN - 1] & 1), take);
        found |= take;
        super::super::wipe(&mut seed);
        counter += 1;
    }
    super::super::wipe(&mut work);
    super::super::wipe(&mut value);
    if !bool::from(found) {
        return None;
    }
    let y: FieldElement = Option::from(curve_rhs(&x_sel).sqrt())?;
    let same_parity = Choice::from(y.is_odd().unwrap_u8() ^ odd_sel ^ 1);
    let y = FieldElement::conditional_select(&-y, &y, same_parity);
    point_from_xy(&fe_bytes(&x_sel), &fe_bytes(&y))
}
