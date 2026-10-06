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

/// The two-tap filter: only p0 and q0 move.
pub(super) fn filter2(
    px: &mut [u8],
    p: usize,
    step: usize,
    (p1, p0, q0, q1): (i32, i32, i32, i32),
) {
    let a = 3 * (q0 - p0) + sclip1(p1 - q1);
    let (a1, a2) = (sclip2((a + 4) >> 3), sclip2((a + 3) >> 3));
    px[p - step] = (p0 + a2).clamp(0, 255) as u8;
    px[p] = (q0 - a1).clamp(0, 255) as u8;
}

pub(super) fn sclip1(v: i32) -> i32 {
    v.clamp(-128, 127)
}

pub(super) fn sclip2(v: i32) -> i32 {
    v.clamp(-16, 15)
}
