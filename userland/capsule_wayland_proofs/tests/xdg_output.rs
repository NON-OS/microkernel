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

//! An xdg_output's details ended by its own done below version 3, and from
//! 3 by its wl_output's done, as the protocol moved it.

use nonos_capsule_wayland_proofs::client::*;

const OUTPUT: u32 = 30;
const MANAGER: u32 = 31;
const XDG_OUTPUT: u32 = 32;

fn made(version: u32) -> Vec<Ev> {
    let mut c = Client::new();
    c.bind(REGISTRY, &[("wl_output", 4, OUTPUT), ("zxdg_output_manager_v1", version, MANAGER)]);
    c.events();
    c.req(MANAGER, 1, &[A::U(XDG_OUTPUT), A::U(OUTPUT)]);
    c.events()
}

#[test]
fn below_3_its_own_done_ends_it() {
    let evs = made(2);
    let ops: Vec<u16> = to(&evs, XDG_OUTPUT).iter().map(|e| e.opcode).collect();
    // logical_position, logical_size, name, description, done.
    assert_eq!(ops, vec![0, 1, 3, 4, 2]);
    assert!(to(&evs, OUTPUT).is_empty());
}

#[test]
fn from_3_the_wl_outputs_done_ends_it() {
    let evs = made(3);
    let ops: Vec<u16> = to(&evs, XDG_OUTPUT).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![0, 1, 3, 4]);
    let out: Vec<u16> = to(&evs, OUTPUT).iter().map(|e| e.opcode).collect();
    assert_eq!(out, vec![2], "wl_output.done");
    assert!(
        evs.iter().position(|e| e.object == OUTPUT)
            > evs.iter().rposition(|e| e.object == XDG_OUTPUT)
    );
}

/// A surface destroyed is off every output, so its id given to a new surface
/// enters them again when that one is shown.
#[test]
fn a_destroyed_surface_leaves_the_outputs_list() {
    let mut c = toplevel(6);
    c.bind(40, &[("wl_output", 4, OUTPUT)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (64, 64));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    assert!(c.s.guest.scene.on_output.contains(&SURFACE), "shown, on the output");
    c.req(TOPLEVEL, 0, &[]);
    c.req(XDG_SURFACE, 0, &[]);
    c.req(SURFACE, 0, &[]);
    c.events();
    assert!(!c.s.guest.scene.on_output.contains(&SURFACE), "gone with the surface");
}
