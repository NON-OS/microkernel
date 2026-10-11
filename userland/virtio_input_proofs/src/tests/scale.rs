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

//! libc's absolute-range scaling, which every absolute device goes through.

use nonos_libc::{input_abs_scale, INPUT_ABS_MAX};

#[test]
fn the_ends_of_any_axis_are_the_ends_of_the_range() {
    for (min, max) in [(0, 0x7FFF), (0, 0xFFFF), (-100, 100), (1, 2), (i32::MIN, i32::MAX)] {
        assert_eq!(input_abs_scale(min, min, max), 0);
        assert_eq!(input_abs_scale(max, min, max), INPUT_ABS_MAX);
        assert_eq!(input_abs_scale(min.saturating_sub(1), min, max), 0, "clamped low");
        assert_eq!(input_abs_scale(max.saturating_add(1), min, max), INPUT_ABS_MAX, "clamped high");
    }
}

#[test]
fn the_tablet_range_is_left_as_it_is_and_a_flat_axis_is_zero() {
    for v in [0, 1, 1234, 0x4000, 0x7FFE, 0x7FFF] {
        assert_eq!(input_abs_scale(v, 0, 0x7FFF), v);
    }
    assert_eq!(input_abs_scale(5, 3, 3), 0);
    assert_eq!(input_abs_scale(5, 9, 3), 0);
    // Rounded down: 0x8000 is 16383.75 of the way across.
    assert_eq!(input_abs_scale(0x8000, 0, 0xFFFF), 0x3FFF);
}
