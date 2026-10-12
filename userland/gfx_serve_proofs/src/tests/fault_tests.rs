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

//! The fault path: a lost output gets its mode back on its current
//! primary before STATE answers, keeps its serial, and is tried again on
//! every STATE while putting the mode back fails.

use super::output_tests::{surface, two, Two};
use crate::model::wire::{ask, u32s, COMPOSITOR};
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::{ops, State};

fn state(s: &mut Two, o: u16) -> State {
    let (st, body) = ask(s, COMPOSITOR, ops::STATE, o, &[]);
    assert_eq!(st, 0);
    State::decode(&body).unwrap()
}

#[test]
fn a_lost_output_is_shown_again_from_its_primary_before_the_answer() {
    let mut s = two();
    let b = surface(&mut s, 1, COMPOSITOR);
    let base = s.slots[1].current.unwrap().region.base;
    s.platform.memory.get_mut(&base).unwrap()[..4].copy_from_slice(&[9, 8, 7, 6]);
    s.display.heads[1].lost = true;
    s.display.heads[1].fb.fill(0);
    let st = state(&mut s, 1);
    assert_eq!((st.flags, st.serial), (LOST | SCANNING_OUT, b.serial));
    assert_eq!((s.display.heads[1].sets, s.display.heads[1].fb[0]), (1, 0x0607_0809));
    assert_eq!(s.display.heads[0].sets, 0, "the other output untouched");
    assert_eq!(state(&mut s, 1).flags, SCANNING_OUT, "reported once");
    let flush = u32s(&[b.serial, 1, 0, 0, 8, 8]);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::FLUSH, 1, &flush).0, 0, "same primary, same serial");
}

#[test]
fn while_the_mode_will_not_come_back_every_state_says_lost_and_tries_again() {
    let mut s = two();
    surface(&mut s, 0, COMPOSITOR);
    s.display.fail_set = true;
    s.display.heads[0].lost = true;
    assert_eq!(state(&mut s, 0).flags, LOST, "not scanning out");
    assert_eq!(state(&mut s, 0).flags, LOST, "still lost though the device said it once");
    s.display.fail_set = false;
    assert_eq!(state(&mut s, 0).flags, LOST | SCANNING_OUT);
    assert_eq!(s.display.heads[0].sets, 1);
    assert_eq!(state(&mut s, 0).flags, SCANNING_OUT);
}

#[test]
fn a_loss_before_any_surface_is_passed_on_with_nothing_to_put_back() {
    let mut s = two();
    s.display.heads[1].lost = true;
    s.display.fail_set = true;
    assert_eq!(state(&mut s, 1).flags, LOST | SCANNING_OUT);
    assert!(!s.slots[1].lost && s.display.heads[1].sets == 0);
    assert_eq!(state(&mut s, 1).flags, SCANNING_OUT);
}
