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

//! C8u2: a client places a subsurface only on its own surfaces, never on
//! another client's under any id, and only where may.rs allows.

use crate::wayland_object::{Object, Objects};
use crate::wayland_sub::may::may_place;
use crate::wayland_sub::order::Stack;
use crate::wayland_sub::state::{Sub, Subs};

fn client(surfaces: &[u32]) -> Objects {
    let mut o = Objects::new();
    assert!(surfaces.iter().all(|s| o.put(*s, Object::Surface)));
    o
}

#[test]
fn another_client_s_surface_is_never_a_parent() {
    let (mine, theirs) = (client(&[3, 4]), client(&[9]));
    let own = |id| mine.get(id) == Some(Object::Surface);
    let subs = Subs::default();
    assert!(theirs.get(9) == Some(Object::Surface) && !may_place(&own, false, &subs, 3, 9));
    assert!(!may_place(&own, false, &subs, 9, 3), "nor theirs placed on mine");
    assert!(may_place(&own, false, &subs, 3, 4), "two of its own");
    assert!(!may_place(&own, false, &subs, 3, 3) && !may_place(&own, true, &subs, 3, 4));
    let placed = Sub {
        id: 7,
        surface: 4,
        parent: 3,
        at: (0, 0),
        next_at: None,
        sync: true,
        shown: None,
        waiting: None,
    };
    let subs = Subs {
        list: vec![placed],
        stacks: vec![Stack { parent: 3, ids: vec![3, 4] }],
        ..Subs::default()
    };
    assert!(!may_place(&own, false, &subs, 4, 3), "4 has its role");
    assert!(!may_place(&own, false, &subs, 3, 4), "3 under 4 under 3");
}
