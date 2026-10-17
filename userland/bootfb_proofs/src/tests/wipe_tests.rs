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
//! The firmware framebuffer at bring-up: a dead driver's picture is
//! zeroed before the first flush, row by row inside the record's mapping,
//! and nothing before the first pixel is written.

use super::firmware_model::Model;
use crate::bring_up::bring_up;
use crate::geometry::BGRX;

#[test]
fn a_dead_driver_s_picture_is_zeroed_at_bring_up() {
    let (mut m, r) = Model::new(64, 16, 512, BGRX, 0x40);
    m.bar.fill(0x5A5A_5A5A);
    let fb = bring_up(&mut m, &r).unwrap();
    for y in 0..16 {
        let row = 0x40 + y * 512;
        assert!((row..row + 64 * 4).all(|i| m.byte(i) == 0), "row {y}");
    }
    // The bar ends at the last row's last pixel; the rows above it pad.
    assert!((0..15).all(|y| m.byte(0x40 + y * 512 + 64 * 4) == 0x5A), "padding is no pixel");
    assert!((0..0x40).all(|i| m.byte(i) == 0x5A), "before the first pixel");
    assert_eq!(fb.height, 16);
}
