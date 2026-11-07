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


//! A new id that names a live object ends the client, as libwayland does:
//! put over it, the old object's state stayed in the scene with nothing to
//! free it, and a client could grow it without bound.

use nonos_capsule_wayland_proofs::client::*;

const DISPLAY_ERROR: u16 = 0;
const CREATE_SURFACE: u16 = 0;
const CREATE_REGION: u16 = 1;
const REGION_DESTROY: u16 = 0;
const X: u32 = 50;

fn compositor() -> Client {
    let mut c = Client::new();
    c.bind(REGISTRY, &[("wl_compositor", 6, COMPOSITOR)]);
    c.events();
    c
}

#[test]
fn a_live_id_given_again_is_fatal() {
    let mut c = compositor();
    c.req(COMPOSITOR, CREATE_SURFACE, &[A::U(X)]);
    c.req(COMPOSITOR, CREATE_REGION, &[A::U(X)]);
    let evs = c.events();
    let err = to(&evs, 1).into_iter().find(|e| e.opcode == DISPLAY_ERROR).expect("an error");
    assert_eq!(err.word(1), 0, "invalid_object");
    assert!(c.s.guest.objects.failed(), "the client is ended");
}

#[test]
fn an_id_is_free_again_after_its_delete_id() {
    let mut c = compositor();
    c.req(COMPOSITOR, CREATE_REGION, &[A::U(X)]);
    c.req(X, REGION_DESTROY, &[]);
    c.req(COMPOSITOR, CREATE_SURFACE, &[A::U(X)]);
    let evs = c.events();
    assert!(to(&evs, 1).iter().all(|e| e.opcode != DISPLAY_ERROR), "no error");
    assert!(!c.s.guest.objects.failed());
    assert_eq!(c.s.guest.scene.surfaces.iter().filter(|s| s.id == X).count(), 1);
}

/// A wl_surface.attach naming an object that is no buffer ends the client;
/// kept, a buffer release went to that object.
#[test]
fn an_attach_of_no_buffer_is_fatal() {
    let mut c = toplevel(6);
    c.req(SURFACE, 1, &[A::U(KEYBOARD), A::I(0), A::I(0)]);
    let evs = c.events();
    assert!(to(&evs, 1).iter().any(|e| e.opcode == DISPLAY_ERROR), "an error");
    assert!(c.s.guest.objects.failed());
}

#[test]
fn an_attach_of_null_unmaps_as_before() {
    let mut c = toplevel(6);
    c.req(SURFACE, 1, &[A::U(0), A::I(0), A::I(0)]);
    c.events();
    assert!(!c.s.guest.objects.failed());
}
