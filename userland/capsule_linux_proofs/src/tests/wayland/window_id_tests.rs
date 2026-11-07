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
//! A window brought to `Scene.win`, to be acted on; the one there goes

//! A guest's windows each have a window manager id of their own
//! (wayland/window_front/ids.rs). They were taken from the kernel's surface
//! handle, whose low word is the same for every surface, so a dialog had its
//! parent's id and its keys and clicks went to the parent.

use crate::window_ids::{free_id, GUEST_POPUPS, GUEST_WINDOWS};

/// Two handles as a booted image gives them.
const HANDLES: [u64; 2] = [0xA_0000_0001, 0xB_0000_0001];

#[test]
fn the_old_id_was_the_same_for_two_surfaces() {
    let old = |h: u64| GUEST_WINDOWS | (h as u32 & 0xFFFF);
    assert_eq!(old(HANDLES[0]), old(HANDLES[1]));
}

#[test]
fn a_second_window_gets_an_id_the_first_does_not_have() {
    let first = free_id(GUEST_WINDOWS, &[]).expect("free");
    let second = free_id(GUEST_WINDOWS, &[first]).expect("free");
    assert_ne!(first, second);
    assert_eq!((first, second), (GUEST_WINDOWS | 1, GUEST_WINDOWS | 2));
}

#[test]
fn an_id_given_up_is_given_again_and_the_ranges_stay_apart() {
    let used = [GUEST_WINDOWS | 1, GUEST_WINDOWS | 3];
    assert_eq!(free_id(GUEST_WINDOWS, &used), Some(GUEST_WINDOWS | 2));
    assert_eq!(free_id(GUEST_POPUPS, &used), Some(GUEST_POPUPS | 1));
    assert_ne!(GUEST_WINDOWS & 0xFFFF_0000, GUEST_POPUPS & 0xFFFF_0000);
}

#[test]
fn every_id_taken_is_none() {
    let all: Vec<u32> = (1..=0xFFFF).map(|n| GUEST_WINDOWS | n).collect();
    assert_eq!(free_id(GUEST_WINDOWS, &all), None);
}
