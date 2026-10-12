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

//! Malformed commits, and the group's constants.

use nonos_wifi_core::sae::frame::{parse_commit, parse_token_request};

use super::known::{a32, h, PEER_COMMIT};

#[test]
fn malformed_commits_are_refused() {
    let good = h(PEER_COMMIT);
    assert!(parse_commit(&good).is_some());
    assert!(parse_commit(&good[..good.len() - 1]).is_none(), "short");
    let mut g = good.clone();
    g[0] = 20;
    assert!(parse_commit(&g).is_none(), "group 20 is not run");
    for scalar in [[0u8; 32], {
        let mut one = [0u8; 32];
        one[31] = 1;
        one
    }, [0xFF; 32]] {
        let mut s = good.clone();
        s[2..34].copy_from_slice(&scalar);
        assert!(parse_commit(&s).is_none(), "scalar must be in (1, r)");
    }
    let mut off = good.clone();
    off[97] ^= 1; // flip a bit of y: no longer on the curve
    assert!(parse_commit(&off).is_none(), "element off the curve");
    let mut big = good.clone();
    big[34..66].copy_from_slice(&[0xFF; 32]);
    assert!(parse_commit(&big).is_none(), "x not below p");
    // Token requests: wrong group, empty, a container that runs past the body.
    assert!(parse_token_request(&[0x14, 0x00, 1, 2], false).is_none());
    assert!(parse_token_request(&[0x13, 0x00], false).is_none());
    assert!(parse_token_request(&[0x13, 0x00, 255, 40, 93, 1, 2], true).is_none());
}

/// The group's constants, decoded from hex at compile time, against what
/// holds between them and the published P-256 values: 2^256 mod p plus p
/// wraps to zero, and b and the order read back as published.
#[test]
fn the_group_constants_are_p256s() {
    use nonos_wifi_core::sae::group::{CURVE_B, ORDER, PRIME, TWO_256_MOD_P};
    let mut carry = 0u16;
    for i in (0..32).rev() {
        let s = PRIME[i] as u16 + TWO_256_MOD_P[i] as u16 + carry;
        assert_eq!(s & 0xff, 0, "byte {i} of p + (2^256 mod p)");
        carry = s >> 8;
    }
    assert_eq!(carry, 1, "p + (2^256 mod p) is 2^256");
    assert_eq!(a32(&h("ffffffff00000001000000000000000000000000ffffffffffffffffffffffff")), PRIME);
    assert_eq!(a32(&h("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551")), ORDER);
    assert_eq!(a32(&h("5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b")), CURVE_B);
}
