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

/// BT.601 limited-range YUV to RGB in the 14-bit fixed point libwebp uses.
pub(super) fn rgb(y: i32, u: i32, v: i32) -> u32 {
    let hi = |a: i32, k: i32| (a * k) >> 8;
    let clip = |a: i32| {
        if a & !16383 == 0 {
            (a >> 6) as u32
        } else if a < 0 {
            0
        } else {
            255
        }
    };
    let yy = hi(y, 19077);
    let r = clip(yy + hi(v, 26149) - 14234);
    let g = clip(yy - hi(u, 6419) - hi(v, 13320) + 8708);
    let b = clip(yy + hi(u, 33050) - 17685);
    (r << 16) | (g << 8) | b
}
