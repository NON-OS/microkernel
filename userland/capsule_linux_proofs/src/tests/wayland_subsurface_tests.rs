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

//! wl_subcompositor and wl_subsurface, against wayland.xml, and the
//! subsurface requests the route takes without effect.

use crate::ops::req;
use crate::wayland_object::{Object, Objects};

#[test]
fn subcompositor_and_subsurface_opcodes_are_the_protocols() {
    assert_eq!((req::SUBCOMPOSITOR_DESTROY, req::SUBCOMPOSITOR_GET_SUBSURFACE), (0, 1));
    // destroy, set_position, place_above, place_below, set_sync, set_desync
    assert_eq!((req::SUBSURFACE_DESTROY, req::SUBSURFACE_SET_POSITION), (0, 1));
    assert_eq!(req::SUBSURFACE_SET_DESYNC, 5);
    let no_effect = req::SUBSURFACE_SET_POSITION..=req::SUBSURFACE_SET_DESYNC;
    assert!(no_effect.clone().all(|op| op != req::SUBSURFACE_DESTROY));
    assert_eq!(no_effect.count(), 5);
}

#[test]
fn a_subsurface_is_not_a_surface() {
    let mut objects = Objects::new();
    assert!(objects.put(9, Object::Surface) && objects.put(10, Object::Subsurface));
    assert!(objects.get(10) == Some(Object::Subsurface));
    assert!(objects.get(10) != objects.get(9));
}
