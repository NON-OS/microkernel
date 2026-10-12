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

//! Each bound interface keeps its own keys. A report from one keyboard
//! used to release every key another held and drop its Shift: an idle
//! report from a second keyboard turned a held Shift+A into a release and a
//! second press, a doubled character at a password prompt.

use super::feeding::feed;
use crate::descriptors::HidKind;
use crate::endpoints::ep;
use crate::state::State;
use nonos_libc::{reset, take_posted, INPUT_KIND_KEY_DOWN};

const SHIFT_A: [u8; 8] = [0x02, 0, 0x04, 0, 0, 0, 0, 0];

#[test]
fn a_second_keyboard_idling_does_not_release_the_first_keyboards_keys() {
    reset();
    let mut state = State::new();
    let (mut a, mut b) = (ep(1, 1, 3, HidKind::Keyboard), ep(2, 2, 3, HidKind::Keyboard));
    feed(&mut state, &mut a, &SHIFT_A);
    for _ in 0..10 {
        feed(&mut state, &mut b, &[0; 8]);
    }
    feed(&mut state, &mut a, &SHIFT_A);
    let got = take_posted();
    assert_eq!(
        got.iter().map(|e| (e.kind, e.code, e.flags)).collect::<Vec<_>>(),
        vec![(INPUT_KIND_KEY_DOWN, u32::from(b'A'), 1)],
        "{got:?}"
    );
    assert_eq!(state.key_reports, 12);
}

#[test]
fn shift_held_on_one_keyboard_is_not_dropped_by_anothers_typing() {
    reset();
    let mut state = State::new();
    let (mut a, mut b) = (ep(1, 1, 3, HidKind::Keyboard), ep(2, 2, 3, HidKind::Keyboard));
    feed(&mut state, &mut a, &[0x02, 0, 0, 0, 0, 0, 0, 0]);
    feed(&mut state, &mut b, &[0, 0, 0x05, 0, 0, 0, 0, 0]);
    feed(&mut state, &mut b, &[0; 8]);
    feed(&mut state, &mut a, &SHIFT_A);
    let downs: Vec<u32> = take_posted().iter().filter(|e| e.kind == 0).map(|e| e.code).collect();
    assert_eq!(downs, vec![u32::from(b'b'), u32::from(b'A')], "B's own b, then A's Shift+A");
}
