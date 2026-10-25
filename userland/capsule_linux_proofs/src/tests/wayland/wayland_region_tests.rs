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

//! The region, surface, pool and seat opcodes the server takes, against
//! wayland.xml: a request is served by its number, so a wrong one serves
//! another.

use crate::ops::{req, seat};
use crate::wayland_object::{Object, Objects};

#[test]
fn region_and_surface_opcodes_are_the_protocols() {
    assert_eq!((req::COMPOSITOR_CREATE_SURFACE, req::COMPOSITOR_CREATE_REGION), (0, 1));
    assert_eq!((req::REGION_DESTROY, req::REGION_ADD, req::REGION_SUBTRACT), (0, 1, 2));
    assert_eq!((req::POOL_CREATE_BUFFER, req::POOL_DESTROY, req::POOL_RESIZE), (0, 1, 2));
    let surface = [
        req::SURFACE_DESTROY,
        req::SURFACE_ATTACH,
        req::SURFACE_DAMAGE,
        req::SURFACE_FRAME,
        req::SURFACE_SET_OPAQUE_REGION,
        req::SURFACE_SET_INPUT_REGION,
        req::SURFACE_COMMIT,
        req::SURFACE_SET_BUFFER_TRANSFORM,
        req::SURFACE_SET_BUFFER_SCALE,
        req::SURFACE_DAMAGE_BUFFER,
    ];
    assert_eq!(surface, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn a_region_is_an_object_of_its_own_kind() {
    let mut objects = Objects::new();
    assert!(objects.put(7, Object::Region));
    assert!(objects.get(7) == Some(Object::Region));
    objects.drop_id(7);
    assert!(objects.get(7).is_none());
}

#[test]
fn seat_pointer_keyboard_and_touch() {
    let seat =
        [req::SEAT_GET_POINTER, req::SEAT_GET_KEYBOARD, seat::SEAT_GET_TOUCH, seat::SEAT_RELEASE];
    assert!(seat.iter().copied().eq(0..4));
    assert_eq!((seat::POINTER_SET_CURSOR, seat::POINTER_RELEASE), (0, 1));
    assert_eq!((seat::KEYBOARD_RELEASE, seat::TOUCH_RELEASE), (0, 0));
}
