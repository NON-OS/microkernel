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

//! The version each object is made at: a bound global's own, and for what a
//! request makes, that of the object the request was sent to. And a client
//! told of a fatal error holds nothing and is given nothing more. And the
//! objects of one kind, which a surface entering the outputs reads.

use crate::wayland_object::{Object, Objects};

#[test]
fn a_bound_global_has_the_version_the_client_asked_for() {
    let mut objects = Objects::new();
    assert!(objects.bound(5, Object::Seat, 4));
    assert!(objects.at_least(5, 4));
    assert!(!objects.at_least(5, 5), "a seat bound at 4 is sent nothing from 5");
    assert!(objects.at_least(1, 1) && !objects.at_least(1, 2), "the display is 1");
    assert!(!objects.at_least(9, 1), "an id never made is sent nothing");
}

#[test]
fn what_a_request_makes_takes_the_version_of_the_object_it_was_sent_to() {
    let mut objects = Objects::new();
    assert!(objects.bound(5, Object::Seat, 5));
    assert!(objects.bound(6, Object::Output, 2));
    objects.serving(5);
    assert!(objects.put(7, Object::Keyboard));
    objects.serving(6);
    assert!(objects.put(8, Object::XdgOutput));
    assert!(objects.at_least(7, 5) && !objects.at_least(7, 6));
    assert!(objects.at_least(8, 2) && !objects.at_least(8, 3));
    objects.serving(1);
    assert!(objects.put(9, Object::Registry));
    assert!(!objects.at_least(9, 2), "what the display makes is 1");
}

#[test]
fn a_failed_client_holds_only_the_display_and_is_given_nothing() {
    let mut objects = Objects::new();
    assert!(objects.bound(5, Object::Seat, 5));
    assert!(!objects.failed());
    objects.fail();
    assert!(objects.failed());
    assert!(objects.get(5).is_none());
    assert!(objects.get(1) == Some(Object::Display));
    assert!(!objects.put(6, Object::Surface), "nothing is made after");
    assert!(!objects.bound(7, Object::Seat, 1));
    assert!(objects.get(6).is_none() && objects.get(7).is_none());
}

#[test]
fn all_names_every_object_of_a_kind_oldest_first() {
    let mut objects = Objects::new();
    assert!(objects.bound(9, Object::Output, 2));
    assert!(objects.bound(4, Object::Seat, 5));
    assert!(objects.bound(6, Object::Output, 1));
    assert_eq!(objects.all(Object::Output).collect::<alloc::vec::Vec<_>>(), [9, 6]);
    objects.drop_id(9);
    assert_eq!(objects.all(Object::Output).collect::<alloc::vec::Vec<_>>(), [6]);
}
