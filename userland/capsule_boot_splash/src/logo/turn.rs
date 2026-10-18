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

//! Quarter turns. The firmware may show its logo turned clockwise by 90, 180
//! or 270 degrees (the BGRT status field's orientation offset, ACPI 6.2 and
//! later; read here as clockwise, unverified against the specification's
//! text). The splash shows it as the firmware did, so each shown pixel is
//! looked up in the file through the inverse turn.

/// The logo's size on screen for an image `w` by `h` turned `t` times.
pub fn dims(w: u32, h: u32, t: u8) -> (u32, u32) {
    if t & 1 == 1 {
        (h, w)
    } else {
        (w, h)
    }
}

/// The image pixel shown at (`u`, `v`) of the turned logo.
pub fn source(u: u32, v: u32, w: u32, h: u32, t: u8) -> (u32, u32) {
    match t & 3 {
        0 => (u, v),
        // A clockwise turn sends the image's top left to the top right.
        1 => (v, h - 1 - u),
        2 => (w - 1 - u, h - 1 - v),
        _ => (w - 1 - v, u),
    }
}
