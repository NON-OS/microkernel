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

//! Noise in every extension byte, as each kind of extension.

use super::hostile_tests::{exercise, three_blocks};
use crate::build::seal;

#[test]
fn noise_in_every_extension_byte() {
    let mut x: u32 = 0x1234_5678;
    for round in 0..4000 {
        let mut raw = three_blocks();
        for b in raw[128..].iter_mut() {
            x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *b = (x >> 24) as u8;
        }
        raw[128] = [0x02, 0x70, 0xF0][round % 3];
        seal(&mut raw[128..256]);
        seal(&mut raw[256..384]);
        exercise(&raw);
    }
}
