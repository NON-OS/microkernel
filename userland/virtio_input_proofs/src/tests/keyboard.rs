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

//! Keys as the PS/2 driver posts them: resolved through the layout with
//! shift and caps, a release paired with its press's code, and the keypad
//! left alone.

use nonos_keymap::Layout;
use nonos_libc::{posted, InputEvent};

use crate::keyboard::{set_layout, Keyboard};

const A: u16 = 30;
const LSHIFT: u16 = 42;

pub fn keys() -> Vec<(u16, u16, u32)> {
    posted().iter().map(|e: &InputEvent| (e.kind, e.flags, e.code)).collect()
}

#[test]
fn shift_and_caps_case_a_letter_and_the_release_keeps_its_code() {
    let mut kb = Keyboard::new();
    kb.key(A, 1);
    kb.key(A, 0);
    assert_eq!(keys(), vec![(0, 0, 'a' as u32), (1, 0, 'a' as u32)]);
    kb.key(LSHIFT, 1);
    kb.key(A, 1);
    kb.key(LSHIFT, 0);
    kb.key(A, 0);
    let got = keys();
    assert_eq!(got[1], (0, 1, 'A' as u32));
    assert_eq!(got[3], (1, 0, 'A' as u32), "released as it was pressed");
    kb.key(58, 1); // Caps Lock
    kb.key(58, 0);
    kb.key(A, 1);
    assert_eq!(keys().last(), Some(&(0, 0x10, 'A' as u32)));
}

#[test]
fn the_layout_moves_letters_but_not_the_keypad() {
    let mut kb = Keyboard::new();
    set_layout(Layout::Fr);
    kb.key(16, 1); // the key US calls Q is A on AZERTY
    kb.key(79, 1); // keypad 1
    kb.key(2, 1); // main row 1 is & on AZERTY
    let got: Vec<u32> = keys().iter().map(|k| k.2).collect();
    assert_eq!(got, vec!['a' as u32, '1' as u32, '&' as u32]);
}
