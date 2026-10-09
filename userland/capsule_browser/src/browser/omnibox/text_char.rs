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

use nonos_app_skeleton::{MOD_ALT, MOD_CTRL, MOD_META};

/* The character a key-down event types, if any.
 *
 * The keyboard drivers resolve layout, shift, caps lock and AltGr before
 * the event leaves them, so `code` already is the final character and is
 * taken as it is: shifting it again turned a German Shift+7 '/' into '?'.
 * A held Ctrl, Alt or Meta makes the key a shortcut rather than text
 * (AltGr, a separate flag, still types). Control codes, the C1 range and
 * the driver's navigation and function codes type nothing. */
pub fn text_char(code: u32, flags: u16) -> Option<char> {
    if flags & (MOD_CTRL | MOD_ALT | MOD_META) != 0 {
        return None;
    }
    if code < 0x20 || (0x7F..0xA0).contains(&code) || (0x1000..=0x1FFF).contains(&code) {
        return None;
    }
    char::from_u32(code)
}
