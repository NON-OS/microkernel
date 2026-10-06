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

//! Paste, mouse reports and motion, as a program reads them.

use nonos_vt::input::{encode_mouse, encode_paste, Button, Mods, MouseEvent, MouseKind};
use nonos_vt::{Modes, MouseMode};

const NONE: Mods = Mods { shift: false, alt: false, ctrl: false };

#[test]
fn paste_cannot_break_out_of_its_bracket() {
    let m = Modes { bracketed_paste: true, ..Modes::default() };
    let mut v = Vec::new();
    encode_paste("ls\x1b[201~\nrm -rf ~\r\n", &m, &mut v);
    assert_eq!(v, b"\x1b[200~ls[201~\rrm -rf ~\r\x1b[201~");
}

#[test]
fn mouse_reports_only_when_asked() {
    let ev =
        MouseEvent { kind: MouseKind::Press, button: Button::Left, col: 4, row: 2, mods: NONE };
    let mut v = Vec::new();
    assert!(!encode_mouse(&ev, &Modes::default(), &mut v));
    let m = Modes { mouse: MouseMode::Click, mouse_sgr: true, ..Modes::default() };
    assert!(encode_mouse(&ev, &m, &mut v));
    assert_eq!(v, b"\x1b[<0;5;3M");
    let m = Modes { mouse: MouseMode::Click, ..Modes::default() };
    let mut v = Vec::new();
    encode_mouse(&ev, &m, &mut v);
    assert_eq!(v, [0x1b, b'[', b'M', 32, 37, 35]);
}

#[test]
fn motion_without_a_button_needs_any_motion_mode() {
    let ev =
        MouseEvent { kind: MouseKind::Motion, button: Button::None, col: 0, row: 0, mods: NONE };
    let mut v = Vec::new();
    let drag = Modes { mouse: MouseMode::Drag, mouse_sgr: true, ..Modes::default() };
    assert!(!encode_mouse(&ev, &drag, &mut v));
    let any = Modes { mouse: MouseMode::Motion, mouse_sgr: true, ..Modes::default() };
    assert!(encode_mouse(&ev, &any, &mut v));
    assert_eq!(v, b"\x1b[<35;1;1M");
}
