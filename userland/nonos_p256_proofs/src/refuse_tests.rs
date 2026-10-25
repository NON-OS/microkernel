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

//! Each refusal on its own input: a secret of 0 or at or above r, a
//! coordinate at or above p, a point off the curve, and the identity.

use crate::curve_tests::{GX, GY};
use crate::hex::arr;
use nonos_p256::*;

#[test]
fn secrets_outside_one_to_r_minus_one() {
    assert!(Secret::from_bytes(&[0u8; 32]).is_none(), "zero");
    assert!(Secret::from_bytes(&ORDER).is_none(), "r");
    assert!(Secret::from_bytes(&[0xff; 32]).is_none(), "2^256 - 1");
    let mut r1 = ORDER;
    r1[LEN - 1] -= 1;
    assert!(Secret::from_bytes(&r1).is_some(), "r - 1");
    let mut one = [0u8; 32];
    one[LEN - 1] = 1;
    assert!(Secret::from_bytes(&one).is_some(), "1");
    assert!(scalar_is_trivial(&scalar_from_bytes(&one).unwrap()));
}

#[test]
fn points_off_the_curve_or_out_of_the_field() {
    let (x, y) = (arr::<32>(GX), arr::<32>(GY));
    let mut off = y;
    off[LEN - 1] ^= 1;
    assert!(point_from_xy(&x, &off).is_none(), "y changed by one bit");
    assert!(point_from_xy(&PRIME, &y).is_none(), "x = p");
    assert!(point_from_xy(&x, &[0xff; 32]).is_none(), "y above p");
    assert!(point_from_xy(&[0; 32], &[0; 32]).is_none(), "(0, 0)");
    assert!(point_xy(&ProjectivePoint::IDENTITY).is_none(), "identity");
    assert!(fe_from_bytes_ct(&PRIME).1.unwrap_u8() == 0, "p as a field element");
}
