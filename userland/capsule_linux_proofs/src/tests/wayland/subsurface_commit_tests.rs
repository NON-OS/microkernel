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

//! C8u: what a parent's commit applies to its subsurfaces: a new position,
//! a synchronized commit that waited, and a new place in its stack; and
//! that a parent placed on its own subsurface is caught.

use crate::wayland_sub::blend::Plane;
use crate::wayland_sub::order::Stack;
use crate::wayland_sub::restack::{parent_commit, placed_on};
use crate::wayland_sub::state::{Kept, Sub, Subs};

fn sub(surface: u32, parent: u32, sync: bool) -> Sub {
    let (id, at, next_at) = (surface + 100, (0, 0), Some((7, -3)));
    Sub { id, surface, parent, at, next_at, sync, shown: None, waiting: None }
}

fn kept(bytes: usize) -> Option<Kept> {
    Some(Kept { px: vec![1; bytes], plane: Plane { w: 1, h: 1, stride: 4 } })
}

fn subs() -> Subs {
    let list = vec![sub(2, 1, true), sub(3, 1, false), sub(4, 2, true)];
    let stacks =
        vec![Stack { parent: 1, ids: vec![1, 2, 3] }, Stack { parent: 2, ids: vec![2, 4] }];
    Subs { list, stacks, ..Subs::default() }
}

#[test]
fn a_parent_s_commit_takes_its_own_subsurfaces_waiting_state_only() {
    let mut s = subs();
    s.list[0].waiting = Some(kept(4));
    s.list[2].waiting = Some(kept(8));
    assert_eq!(s.kept(), 12);
    parent_commit(&mut s, 1);
    assert_eq!((s.list[0].at, s.list[1].at, s.list[2].at), ((7, -3), (7, -3), (0, 0)));
    assert!(s.list[0].shown.is_some() && s.list[0].waiting.is_none());
    assert!(s.list[2].waiting.is_some(), "placed on 2, so 2's commit takes it");
    parent_commit(&mut s, 2);
    assert!(s.list[2].shown.is_some() && s.kept() == 12);
}

#[test]
fn place_above_and_below_wait_for_the_parent_and_keep_the_parent_in_its_stack() {
    let mut s = subs();
    s.restack = vec![(3, 1, false), (2, 3, true), (4, 9, true)];
    parent_commit(&mut s, 1);
    assert_eq!(s.stacks[0].ids, vec![3, 2, 1], "3 under 1, then 2 just above 3");
    assert_eq!(s.restack, vec![(4, 9, true)], "not 1's to apply");
    parent_commit(&mut s, 2);
    assert_eq!(s.stacks[1].ids, vec![2, 4], "a reference gone: on top");
    assert!(s.restack.is_empty());
}

#[test]
fn a_parent_placed_on_the_surface_itself_is_caught() {
    let s = subs();
    assert!(placed_on(&s, 4, 2), "4 is on 2");
    assert!(placed_on(&s, 4, 1) && placed_on(&s, 2, 2));
    assert!(!placed_on(&s, 1, 2) && !placed_on(&s, 3, 2));
}
