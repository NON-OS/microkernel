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

use crate::router_chord::is_reserved_chord;

const KEY_DOWN: u16 = 0;
const KEY_UP: u16 = 1;
const ESC: u32 = 0x1B;
const SHIFT: u16 = 1 << 0;
const CTRL: u16 = 1 << 1;
const ALT: u16 = 1 << 2;

#[test]
fn ctrl_alt_esc_is_the_chord() {
    assert!(is_reserved_chord(KEY_DOWN, ESC, CTRL | ALT));
    assert!(is_reserved_chord(KEY_DOWN, ESC, CTRL | ALT | SHIFT), "an extra modifier keeps it");
}

#[test]
fn nothing_less_is_the_chord() {
    assert!(!is_reserved_chord(KEY_DOWN, ESC, 0), "a plain Esc is the window's");
    assert!(!is_reserved_chord(KEY_DOWN, ESC, CTRL));
    assert!(!is_reserved_chord(KEY_DOWN, ESC, ALT));
    assert!(!is_reserved_chord(KEY_DOWN, b'q' as u32, CTRL | ALT));
    assert!(!is_reserved_chord(KEY_UP, ESC, CTRL | ALT), "the release follows its press");
}
