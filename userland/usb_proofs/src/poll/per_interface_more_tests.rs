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

//! Each bound interface repeats its own held key, and keeps its own buttons:
//! one mouse never lets go of another's button mid drag.

use super::feeding::feed;
use crate::descriptors::HidKind;
use crate::endpoints::{ep, posted};
use crate::state::State;
use nonos_libc::{reset, take_posted, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_KEY_DOWN};

#[test]
fn each_keyboard_repeats_its_own_held_key() {
    reset();
    let mut state = State::new();
    let (mut a, mut b) = (ep(1, 1, 3, HidKind::Keyboard), ep(2, 2, 3, HidKind::Keyboard));
    feed(&mut state, &mut a, &[0, 0, 0x04, 0, 0, 0, 0, 0]);
    feed(&mut state, &mut b, &[0, 0, 0x05, 0, 0, 0, 0, 0]);
    // Each repeats once, 500 ms after its own first tick, and no other's.
    for t in 0..=520 {
        a.dev.tick(t);
        b.dev.tick(t);
    }
    let (ka, kb) = (u32::from(b'a'), u32::from(b'b'));
    let down = INPUT_KIND_KEY_DOWN;
    assert_eq!(posted(), vec![(down, ka), (down, kb), (down, ka), (down, kb)]);
}

#[test]
fn a_second_mouse_moving_does_not_end_the_first_mouses_drag() {
    reset();
    let mut state = State::new();
    let (mut a, mut b) = (ep(1, 1, 3, HidKind::Mouse), ep(2, 2, 3, HidKind::Mouse));
    feed(&mut state, &mut a, &[0x01, 0, 0]);
    for _ in 0..10 {
        feed(&mut state, &mut b, &[0, 5, 5]);
    }
    let kinds: Vec<u16> = take_posted().iter().map(|e| e.kind).collect();
    assert_eq!(kinds[0], INPUT_KIND_BUTTON_DOWN);
    assert_eq!(kinds[1..], [nonos_libc::INPUT_KIND_POINTER_REL; 10], "and no BUTTON_UP");
    assert_eq!(state.mouse_reports, 11);
}
