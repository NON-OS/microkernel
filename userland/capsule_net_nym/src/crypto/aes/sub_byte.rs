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

//! The AES S-box computed rather than looked up.

use super::xtime::xtime;

/*
 * A 256 byte table indexed by a state or key byte leaks that byte through
 * the cache: another capsule sharing the core or the last level cache can
 * time which lines were touched and recover the key (Bernstein 2005, Osvik,
 * Shamir and Tromer 2006). So nothing here indexes memory or branches on the
 * input. The byte is inverted in GF(2^8) as x^254 over one fixed addition
 * chain, every multiply is the same eight masked shift-and-add steps, and the
 * FIPS 197 affine map is a handful of rotates. Zero maps to zero, as the
 * standard requires, because 0^254 is 0.
 */
pub(crate) fn sub_byte(input: u8) -> u8 {
    let inverse = invert(input);
    inverse
        ^ inverse.rotate_left(1)
        ^ inverse.rotate_left(2)
        ^ inverse.rotate_left(3)
        ^ inverse.rotate_left(4)
        ^ 0x63
}

fn invert(x: u8) -> u8 {
    let x2 = multiply(x, x);
    let x3 = multiply(x2, x);
    let x12 = square_times(x3, 2);
    let x14 = multiply(x12, x2);
    let x15 = multiply(x12, x3);
    multiply(square_times(x15, 4), x14)
}

fn square_times(mut value: u8, times: u32) -> u8 {
    for _ in 0..times {
        value = multiply(value, value);
    }
    value
}

fn multiply(mut a: u8, mut b: u8) -> u8 {
    let mut product = 0u8;
    for _ in 0..8 {
        product ^= a & 0u8.wrapping_sub(b & 1);
        a = xtime(a);
        b >>= 1;
    }
    product
}
