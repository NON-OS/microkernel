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

//! The curve's numbers and its generator. G is FIPS 186-4, D.1.2.3's,
//! recalled; it is confirmed by lying on the curve and having order r. Its
//! multiples were computed by a separate big-integer implementation.

use crate::hex::arr;
use nonos_p256::*;

pub const GX: &str = "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296";
pub const GY: &str = "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5";

fn xy(x: &str, y: &str) -> [u8; 64] {
    let mut out = [0u8; 64];
    out[..32].copy_from_slice(&arr::<32>(x));
    out[32..].copy_from_slice(&arr::<32>(y));
    out
}

fn times_g(k: &str) -> Option<[u8; 64]> {
    point_xy(&(ProjectivePoint::GENERATOR * scalar_from_bytes(&arr::<32>(k))?))
}

// p + (2^256 mod p) wraps to exactly 2^256, and G satisfies the equation
// with these p and b, so neither constant can carry a mistyped digit.
#[test]
fn constants_and_generator_agree() {
    let (mut carry, mut zero) = (0u16, true);
    for i in (0..LEN).rev() {
        let s = PRIME[i] as u16 + TWO_256_MOD_P[i] as u16 + carry;
        zero &= s & 0xff == 0;
        carry = s >> 8;
    }
    assert!(zero && carry == 1, "p + (2^256 mod p) is 2^256");
    assert_eq!(point_xy(&ProjectivePoint::GENERATOR), Some(xy(GX, GY)));
    let (x, y) = (fe_const(&arr::<32>(GX)), fe_const(&arr::<32>(GY)));
    assert_eq!(y.square(), curve_rhs(&x));
    assert!(point_from_xy(&arr::<32>(GX), &arr::<32>(GY)).is_some());
}

#[test]
fn multiples_of_the_generator() {
    let two_g = xy(
        "7cf27b188d034f7e8a52380304b51ac3c08969e277f21b35a60b48fc47669978",
        "07775510db8ed040293d9ac69f7430dbba7dade63ce982299e04b79d227873d1",
    );
    assert_eq!(times_g(&format!("{:064x}", 2)), Some(two_g));
    let k = "c51e4753afdec1e6b6c6a5b992f43f8dd0c7a8933072708b6522468b2ffb06fd";
    let kg = xy(
        "942c9f408ead9d82d34a1b9a6a827ebe3e2ddf782b448d23be1b6143988ccef4",
        "8c9eaf6c0d14d992fc63bad3e2496be2eee61cb5b97f65f428ca94a5d0ee19a1",
    );
    assert_eq!(times_g(k), Some(kg));
    // (r - 1)G is -G: same x, y = p - Gy. rG is the identity: no encoding.
    let mut r1 = ORDER;
    r1[LEN - 1] -= 1;
    let neg = point_xy(&(ProjectivePoint::GENERATOR * scalar_from_bytes(&r1).unwrap())).unwrap();
    assert_eq!(neg[..32], arr::<32>(GX));
    assert_eq!(fe_const(&neg[32..].try_into().unwrap()), -fe_const(&arr::<32>(GY)));
    assert!(point_xy(&(ProjectivePoint::GENERATOR * scalar_reduce(&ORDER))).is_none());
}
