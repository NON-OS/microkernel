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

//! The config bytes 2, 4 and 6 below a write to an EHCI function, the
//! places its legacy support capability's id (01h) would sit, read only
//! where they could be that capability: at or above 0x40 and outside every
//! capability on the standard list, with the write outside them too.

use super::listed::in_listed;

const FLOOR: u32 = 0x40;
const NONE: [u8; 3] = [0xFF; 3];

/// `before` for Ident, 0xFF where no legacy support id may be.
pub(crate) fn legsup_before(read: &dyn Fn(u16) -> u8, offset: u32) -> [u8; 3] {
    let Ok(at) = u16::try_from(offset) else { return NONE };
    if in_listed(read, at) {
        return NONE;
    }
    let byte = |d: u32| match offset.checked_sub(d).filter(|&o| o >= FLOOR) {
        Some(o) if !in_listed(read, o as u16) => read(o as u16),
        _ => 0xFF,
    };
    [byte(2), byte(4), byte(6)]
}
