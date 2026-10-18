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

//! The keys that paste, and none else.

use nonos_app_skeleton::input::text::is_paste;
use nonos_app_skeleton::{
    InputEvent, InputKind, KEY_INSERT, MOD_ALT, MOD_CAPS, MOD_CTRL, MOD_META, MOD_SHIFT,
};

fn key(code: u32, flags: u16) -> InputEvent {
    InputEvent {
        kind: InputKind::KeyDown,
        flags,
        code,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    }
}

#[test]
fn ctrl_v_and_shift_insert_paste() {
    assert!(is_paste(&key('v' as u32, MOD_CTRL)));
    assert!(is_paste(&key('V' as u32, MOD_CTRL | MOD_CAPS)));
    assert!(is_paste(&key('V' as u32, MOD_CTRL | MOD_SHIFT)));
    assert!(is_paste(&key(KEY_INSERT, MOD_SHIFT)));
}

#[test]
fn nothing_else_pastes() {
    assert!(!is_paste(&key('v' as u32, 0)), "a plain v is a letter");
    assert!(!is_paste(&key('v' as u32, MOD_CTRL | MOD_ALT)));
    assert!(!is_paste(&key('v' as u32, MOD_META)));
    assert!(!is_paste(&key(KEY_INSERT, 0)), "Insert alone");
    assert!(!is_paste(&key(KEY_INSERT, MOD_CTRL | MOD_SHIFT)));
    let mut up = key('v' as u32, MOD_CTRL);
    up.kind = InputKind::KeyUp;
    assert!(!is_paste(&up), "the release does not paste a second time");
}
