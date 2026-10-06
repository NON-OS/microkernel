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

//! Keys, mouse and paste as a program reads them.

use nonos_vt::input::{encode_key, Key, Mods};
use nonos_vt::Modes;

fn key(k: Key, m: Mods, modes: &Modes) -> Vec<u8> {
    let mut v = Vec::new();
    encode_key(k, m, modes, &mut v);
    v
}

const NONE: Mods = Mods { shift: false, alt: false, ctrl: false };
const CTRL: Mods = Mods { shift: false, alt: false, ctrl: true };
const ALT: Mods = Mods { shift: false, alt: true, ctrl: false };

#[test]
fn cursor_keys_follow_the_mode() {
    let mut m = Modes::default();
    assert_eq!(key(Key::Up, NONE, &m), b"\x1b[A");
    m.cursor_keys = true;
    assert_eq!(key(Key::Up, NONE, &m), b"\x1bOA");
    assert_eq!(key(Key::Right, CTRL, &m), b"\x1b[1;5C");
}

#[test]
fn control_and_alt() {
    let m = Modes::default();
    assert_eq!(key(Key::Char('c'), CTRL, &m), [3]);
    assert_eq!(key(Key::Char('['), CTRL, &m), [0x1b]);
    assert_eq!(key(Key::Char('x'), ALT, &m), b"\x1bx");
    assert_eq!(key(Key::Backspace, NONE, &m), [0x7f]);
    assert_eq!(key(Key::Enter, NONE, &m), b"\r");
    assert_eq!(key(Key::Tab, Mods { shift: true, ..NONE }, &m), b"\x1b[Z");
}

#[test]
fn function_and_editing_keys() {
    let m = Modes::default();
    assert_eq!(key(Key::F(1), NONE, &m), b"\x1bOP");
    assert_eq!(key(Key::F(5), NONE, &m), b"\x1b[15~");
    assert_eq!(key(Key::F(12), CTRL, &m), b"\x1b[24;5~");
    assert_eq!(key(Key::PageDown, NONE, &m), b"\x1b[6~");
    assert_eq!(key(Key::Delete, NONE, &m), b"\x1b[3~");
}
