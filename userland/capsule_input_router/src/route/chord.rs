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

//! The one key chord no window may take: Ctrl+Alt+Esc. However a window
//! behaves (a full screen game, a hung app that never draws again, one that
//! swallows every key), the chord goes to the desktop shell, which brings
//! Process Manager forward to end it. It is held before focus is asked for,
//! so a window that has focus never sees it.

/// nonos_libc INPUT_KIND_KEY_DOWN.
const KEY_DOWN: u16 = 0;
/// The escape key's code.
const KEY_ESC: u32 = 0x1B;
/// The modifier bits a key event carries in its flags (app_skeleton
/// input/modifiers.rs MOD_CTRL and MOD_ALT).
const MOD_CTRL: u16 = 1 << 1;
const MOD_ALT: u16 = 1 << 2;

/// Whether a key event is the press of the reserved chord. Shift or any other
/// modifier held with it does not stop it being the chord.
pub fn is_reserved_chord(kind: u16, code: u32, flags: u16) -> bool {
    kind == KEY_DOWN && code == KEY_ESC && flags & MOD_CTRL != 0 && flags & MOD_ALT != 0
}
