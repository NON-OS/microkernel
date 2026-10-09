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

use crate::input::{InputEvent, KEY_INSERT, MOD_ALT, MOD_CTRL, MOD_META, MOD_SHIFT};

/// Whether a key press asks a text field to paste: Ctrl+V, or Shift+Insert,
/// the older binding many keyboards and hands still use. The keymap sends a
/// Ctrl chord's letter as typed, so either case of v counts.
pub fn is_paste(event: &InputEvent) -> bool {
    if !event.is_key_down() || event.flags & (MOD_ALT | MOD_META) != 0 {
        return false;
    }
    let ctrl = event.flags & MOD_CTRL != 0;
    let shift = event.flags & MOD_SHIFT != 0;
    match event.code {
        0x56 | 0x76 => ctrl,
        KEY_INSERT => shift && !ctrl,
        _ => false,
    }
}
