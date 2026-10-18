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

//! Universal plane and cursor register offsets (SKL PRM Vol 2c, PLANE_*
//! and CUR_*). Plane 1 is the primary; planes 2 and 3 step by 0x100.

use super::pipe::Pipe;

fn plane_base(p: Pipe, plane: u32) -> u32 {
    0x7_0180 + 0x1000 * p.index() + 0x100 * (plane - 1)
}

pub fn plane_ctl(p: Pipe, plane: u32) -> u32 {
    plane_base(p, plane)
}
pub fn plane_stride(p: Pipe, plane: u32) -> u32 {
    plane_base(p, plane) + 0x08
}
pub fn plane_pos(p: Pipe, plane: u32) -> u32 {
    plane_base(p, plane) + 0x0C
}
pub fn plane_size(p: Pipe, plane: u32) -> u32 {
    plane_base(p, plane) + 0x10
}
/// Writing PLANE_SURF arms every double-buffered plane register; they
/// all take effect together at the next vertical blank's start.
pub fn plane_surf(p: Pipe, plane: u32) -> u32 {
    plane_base(p, plane) + 0x1C
}
pub fn plane_offset(p: Pipe, plane: u32) -> u32 {
    plane_base(p, plane) + 0x24
}
/// The address the plane is scanning out now, after any latch.
pub fn plane_surflive(p: Pipe, plane: u32) -> u32 {
    plane_base(p, plane) + 0x2C
}

pub fn cur_ctl(p: Pipe) -> u32 {
    0x7_0080 + 0x1000 * p.index()
}
/// Writing CUR_BASE arms the cursor's double-buffered registers.
pub fn cur_base(p: Pipe) -> u32 {
    cur_ctl(p) + 0x04
}
pub fn cur_pos(p: Pipe) -> u32 {
    cur_ctl(p) + 0x08
}
