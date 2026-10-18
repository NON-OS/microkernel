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

//! The ASCII byte of a key press: the control code a key that is not a
//! character posts (Enter, Escape, Backspace, Tab).

use super::resolve::resolve_code;

/// The ASCII byte of a press. Printable keys clamp non-ASCII resolutions
/// (accented letters) to 0; the posted input event carries the full
/// codepoint instead.
pub fn ascii(scancode: u8, modifiers: u8, caps: bool) -> u8 {
    match scancode {
        0x28 => return b'\n',
        0x29 => return 0x1b,
        0x2a => return 0x08,
        0x2b => return b'\t',
        _ => {}
    }
    match resolve_code(scancode, modifiers, caps) {
        c @ 0x20..=0x7E => c as u8,
        _ => 0,
    }
}
