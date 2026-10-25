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

//! The curve's numbers, big-endian as FIPS 186-4, D.1.2.3 prints them.

use p256::{FieldBytes, FieldElement};

/// The length of a field element, a scalar, and each point coordinate.
pub const LEN: usize = 32;

/// The prime p, big-endian.
pub const PRIME: [u8; LEN] =
    hex32("ffffffff00000001000000000000000000000000ffffffffffffffffffffffff");

/// The group order r, big-endian.
pub const ORDER: [u8; LEN] =
    hex32("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");

/// The curve coefficient b, big-endian (a is -3).
pub const CURVE_B: [u8; LEN] =
    hex32("5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b");

/// 2^256 mod p, to reduce a value longer than one field element.
pub const TWO_256_MOD_P: [u8; LEN] =
    hex32("00000000fffffffeffffffffffffffffffffffff000000000000000000000001");

// A big-endian 32-byte constant from 64 hex digits, evaluated at compile
// time for the constants above.
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
/// it, and `nonos_p256_proofs` checks each against the others and the
/// curve, so any other character reads as zero.
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
