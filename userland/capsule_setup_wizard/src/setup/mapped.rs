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

//! What setup's maps and the display it is given may be taken as. The kernel
//! answers a refused map with a negative errno (sys_mmap), never null, so a
//! null check alone takes an error for an address; and a stride shorter
//! than a row of the display would draw each row into the next.

/// The address `mk_mmap` gave back, or `None` when it gave back an error.
pub fn mapped(p: *mut u8) -> Option<u64> {
    let at = p as u64;
    (at != 0 && (at as i64) > 0).then_some(at)
}

/// A display setup can draw on: no side of nothing, and a stride that holds
/// a whole row of 4-byte pixels.
pub fn shape_ok(width: u32, height: u32, stride: u32) -> bool {
    width != 0 && height != 0 && (stride / 4) >= width
}
