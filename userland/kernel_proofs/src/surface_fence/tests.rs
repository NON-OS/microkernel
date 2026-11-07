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

use alloc::vec;
use alloc::vec::Vec;

use super::waiters::{arm_in, disarm_in, forget_in, take_in};
use crate::authorization_tests::token;
use crate::capabilities::Capability;
use crate::syscall::contract::is_allowed;
use crate::syscall::numbers::SyscallNumber::{MkSurfaceFenceRelease, MkSurfaceFenceWait};

#[test]
fn a_release_wakes_every_waiter_on_its_slot_and_no_other() {
    let mut list = Vec::new();
    arm_in(&mut list, 3, 10);
    arm_in(&mut list, 3, 11);
    arm_in(&mut list, 4, 12);
    arm_in(&mut list, 3, 10);
    assert_eq!(take_in(&mut list, 3), [10, 11], "each waiter once, armed twice or not");
    assert_eq!(list, [(4, 12)]);
    assert!(take_in(&mut list, 3).is_empty(), "taken: a second release wakes nobody");
}

#[test]
fn a_waiter_that_gave_up_or_died_is_not_woken() {
    let mut list = Vec::new();
    arm_in(&mut list, 1, 20);
    arm_in(&mut list, 2, 20);
    arm_in(&mut list, 1, 21);
    disarm_in(&mut list, 1, 21);
    assert_eq!(list, [(1, 20), (2, 20)]);
    forget_in(&mut list, 20);
    assert!(list.is_empty(), "an exited thread waits on nothing");
}

#[test]
fn moving_a_fence_takes_the_map_right_and_waiting_the_create_right() {
    let compositor = token(vec![Capability::GraphicsSurfaceMap]);
    assert!(is_allowed(&compositor, MkSurfaceFenceRelease));
    assert!(!is_allowed(&compositor, MkSurfaceFenceWait));
    let app = token(vec![Capability::GraphicsSurfaceCreate]);
    assert!(is_allowed(&app, MkSurfaceFenceWait));
    assert!(!is_allowed(&app, MkSurfaceFenceRelease));
    let none = token(vec![Capability::IPC, Capability::Memory]);
    assert!(!is_allowed(&none, MkSurfaceFenceRelease) && !is_allowed(&none, MkSurfaceFenceWait));
}
