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

//! f5, the MacKey and LTK (Vol 3, Part H, 2.2.7). T is AES-CMAC under the
//! fixed SALT of the DHKey W; each key is then AES-CMAC under T of a
//! counter (0 for MacKey, 1 for LTK), the key ID "btle", N1, N2, A1, A2
//! and the length in bits, 256.

use crate::cmac::aes_cmac;

pub const SALT: [u8; 16] = [
    0x6c, 0x88, 0x83, 0x91, 0xaa, 0xf5, 0xa5, 0x38, 0x60, 0x37, 0x0b, 0xdb, 0x5a, 0x60, 0x83, 0xbe,
];
const KEY_ID: [u8; 4] = *b"btle";
const LENGTH: [u8; 2] = [0x01, 0x00];

/// (MacKey, LTK).
pub fn f5(
    w: &[u8; 32],
    n1: &[u8; 16],
    n2: &[u8; 16],
    a1: &[u8; 7],
    a2: &[u8; 7],
) -> ([u8; 16], [u8; 16]) {
    let t = aes_cmac(&SALT, &[w]);
    let key = |counter: u8| aes_cmac(&t, &[&[counter], &KEY_ID, n1, n2, a1, a2, &LENGTH]);
    (key(0), key(1))
}
