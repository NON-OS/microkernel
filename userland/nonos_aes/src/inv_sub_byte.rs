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

//! The inverse S-box, from the forward one and no table of its own.

use crate::sub_byte::sub_byte;

/*
 * The S-box is S(x) = A(x^-1) for the FIPS 197 affine map A, so a field
 * inverse is u^-1 = A^-1(S(u)) and the inverse S-box is
 * S^-1(y) = (A^-1(y))^-1 = A^-1(S(A^-1(y))). That reuses the computed
 * S-box as it stands, with its fixed chain of masked multiplies, so
 * decryption indexes no table and branches on no state byte either.
 */
pub(crate) fn inv_sub_byte(input: u8) -> u8 {
    inv_affine(sub_byte(inv_affine(input)))
}

// A^-1: three rotates and the constant 0x05 (FIPS 197, 5.3.2).
fn inv_affine(b: u8) -> u8 {
    b.rotate_left(1) ^ b.rotate_left(3) ^ b.rotate_left(6) ^ 0x05
}
