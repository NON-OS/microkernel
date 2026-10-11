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

//! The range an absolute pointer event carries. A `INPUT_KIND_POINTER_ABS`
//! or `INPUT_KIND_TOUCH` event names a point on the whole display as `x`
//! and `y` from 0 to `INPUT_ABS_MAX`; the input router maps that onto the
//! display mode in force, so a driver never needs to know the resolution.
//! Every absolute device (a tablet, a hypervisor's pointer) reports its own
//! range and scales into this one with `input_abs_scale`.

/// The top of the absolute range, the logical maximum of a USB tablet.
pub const INPUT_ABS_MAX: i32 = 0x7FFF;

/// `v` on a device axis running from `min` to `max`, in the absolute range.
/// A value outside the axis is clamped to its end; an axis with no extent
/// (`max <= min`) puts every value at 0.
pub const fn input_abs_scale(v: i32, min: i32, max: i32) -> i32 {
    if max <= min {
        return 0;
    }
    let v = if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
    let span = max as i64 - min as i64;
    ((v as i64 - min as i64) * INPUT_ABS_MAX as i64 / span) as i32
}
