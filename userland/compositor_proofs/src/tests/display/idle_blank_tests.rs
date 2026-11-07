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

//! The idle blank darkens as a closed lid does, one black frame and then
//! nothing presented, and input wakes it whole. The lid and the blank are
//! two reasons for one dark screen: it wakes only when neither holds, and
//! only once.

use super::lid_rig::{black, whole, Rig, Shows};
use crate::damage::Rect;
use crate::state::lid_rule::Lid;

#[test]
fn the_idle_blank_darkens_in_one_frame_and_input_wakes_it_whole() {
    let s = Shows { trusted: true, ..Shows::default() };
    let mut rig = Rig::new(s);
    rig.router(true);
    rig.frame(s);
    assert_eq!((rig.lid, rig.presents), (Lid::Dark, 1));
    assert!(rig.screen == black());
    rig.acc.accumulate(Rect { x: 3, y: 4, width: 50, height: 20 });
    rig.frame(s);
    assert_eq!(rig.presents, 1, "blank: nothing goes out");
    rig.router(false);
    rig.frame(s);
    assert_eq!(rig.lid, Lid::Open);
    assert!(rig.screen == whole(Lid::Open, s), "woken whole");
}

#[test]
fn either_reason_holds_the_screen_dark() {
    let s = Shows::default();
    let mut rig = Rig::new(s);
    rig.shell(true);
    rig.frame(s);
    rig.router(true);
    rig.router(false);
    rig.frame(s);
    assert_eq!((rig.lid, rig.presents), (Lid::Dark, 1), "input does not light a shut lid");
    rig.router(true);
    rig.shell(false);
    rig.frame(s);
    assert_eq!((rig.lid, rig.presents), (Lid::Dark, 1), "opening the lid does not wake a blank");
    rig.router(false);
    rig.frame(s);
    assert_eq!((rig.lid, rig.presents), (Lid::Open, 2), "neither: awake, drawn once");
    assert!(rig.screen == whole(Lid::Open, s));
}

#[test]
fn saying_the_same_again_changes_nothing() {
    let s = Shows::default();
    let mut rig = Rig::new(s);
    rig.router(false);
    rig.frame(s);
    assert_eq!(rig.presents, 0, "awake already: nothing to draw");
    rig.router(true);
    rig.frame(s);
    rig.router(true);
    rig.frame(s);
    assert_eq!(rig.presents, 1);
}
