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


//! What a client can make the server keep through wl_subsurface between two
//! commits of the parent: a bounded number of restacks, still applied.

use nonos_capsule_wayland_proofs::client::*;

/// subsurface/request.rs MOST_RESTACKS.
const MOST_RESTACKS: usize = 256;
const SUBCOMPOSITOR: u32 = 30;
const CHILD: u32 = 50;
const SUB: u32 = 60;
const GET_SUBSURFACE: u16 = 1;
const PLACE_ABOVE: u16 = 2;

#[test]
fn restacks_between_two_commits_are_bounded() {
    let mut c = toplevel(6);
    c.bind(40, &[("wl_subcompositor", 1, SUBCOMPOSITOR)]);
    c.req(COMPOSITOR, 0, &[A::U(CHILD)]);
    c.req(SUBCOMPOSITOR, GET_SUBSURFACE, &[A::U(SUB), A::U(CHILD), A::U(SURFACE)]);
    for _ in 0..10_000 {
        c.req(SUB, PLACE_ABOVE, &[A::U(SURFACE)]);
    }
    c.events();
    assert!(!c.s.guest.objects.failed());
    assert_eq!(c.s.guest.scene.subs.restack.len(), MOST_RESTACKS);
}
