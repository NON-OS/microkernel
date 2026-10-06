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

//! The numeric keypad types what is printed on its keys on every layout.
//! Scan set 1 gives its keys the same ASCII codes as the main row, so
//! resolved through the layout tables keypad 1 typed '&' on French AZERTY
//! and keypad '-' typed 'ß' on German; Linux atkbd gives the keypad its own
//! KP_ keycodes, which no layout remaps. NumLock is not tracked here: the
//! keypad always types its digits, as it did before.

use super::translate::Translated;
use nonos_keymap::Layout;

/// Keypad Divide, the one keypad key behind the E0 prefix that prints a
/// character (E0 1C, keypad Enter, is a control code the layouts pass).
const SCAN_KP_DIVIDE: u8 = 0x35;

/// Whether a set 1 scan code (break bit cleared) is a keypad key: 47 to 53
/// unprefixed, the block from keypad 7 to keypad '.', and E0 35.
pub fn is_keypad(scan: u8, e0: bool) -> bool {
    if e0 {
        scan == SCAN_KP_DIVIDE
    } else {
        (0x47..=0x53).contains(&scan)
    }
}

/// The code a key posts: its own for a keypad key, else the US base
/// resolved through the active layout, shift, caps and AltGr.
pub fn character(t: &Translated, shift: bool, caps: bool, altgr: bool, layout: Layout) -> u32 {
    if t.keypad {
        return t.keycode;
    }
    nonos_keymap::resolve(t.keycode, shift, caps, altgr, layout)
}
