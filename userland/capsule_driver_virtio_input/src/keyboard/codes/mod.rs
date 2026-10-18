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

mod extended;
mod function;
pub mod keycodes;
mod left;
mod right;

pub use keycodes::*;

/// The keycode a Linux key code posts as, as the PS/2 driver would post the
/// same key: a US base character, a control code, or a KEYCODE_ constant.
pub fn keycode_for(code: u16) -> Option<u32> {
    match code {
        0x01..=0x1D => left::keycode_for(code),
        0x1E..=0x39 => right::keycode_for(code),
        0x3A..=0x58 => function::keycode_for(code),
        _ => extended::keycode_for(code),
    }
}

/// Whether a Linux key code is on the numeric keypad: 71 to 83, and
/// KEY_KPSLASH. A keypad key keeps its code under every layout.
pub const fn is_keypad(code: u16) -> bool {
    matches!(code, 71..=83 | 98)
}
