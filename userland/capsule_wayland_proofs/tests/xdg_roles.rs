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


//! The roles xdg-shell holds a surface to, each refused with its protocol
//! error: one xdg_surface a surface, one role object an xdg_surface, and an
//! xdg_surface destroyed only after its toplevel.

use nonos_capsule_wayland_proofs::client::*;

const GET_XDG_SURFACE: u16 = 2;
const GET_TOPLEVEL: u16 = 1;
const XDG_SURFACE_DESTROY: u16 = 0;
const TOPLEVEL_DESTROY: u16 = 0;
const SECOND: u32 = 60;

/// The error the display sent: the object and the code.
fn error(c: &mut Client) -> Option<(u32, u32)> {
    let evs = c.events();
    to(&evs, 1).into_iter().find(|e| e.opcode == 0).map(|e| (e.word(0), e.word(1)))
}

#[test]
fn a_second_xdg_surface_on_a_surface_is_a_role_error() {
    let mut c = toplevel(6);
    c.req(WM_BASE, GET_XDG_SURFACE, &[A::U(SECOND), A::U(SURFACE)]);
    assert_eq!(error(&mut c), Some((WM_BASE, 0)), "xdg_wm_base.error.role");
}

#[test]
fn a_second_toplevel_on_an_xdg_surface_is_already_constructed() {
    let mut c = toplevel(6);
    c.req(XDG_SURFACE, GET_TOPLEVEL, &[A::U(SECOND)]);
    assert_eq!(error(&mut c), Some((XDG_SURFACE, 2)), "xdg_surface.error.already_constructed");
}

#[test]
fn an_xdg_surface_destroyed_before_its_toplevel_is_defunct() {
    let mut c = toplevel(6);
    c.req(XDG_SURFACE, XDG_SURFACE_DESTROY, &[]);
    assert_eq!(error(&mut c), Some((XDG_SURFACE, 6)), "xdg_surface.error.defunct_role_object");
}

#[test]
fn destroyed_in_order_the_surface_takes_a_new_xdg_surface() {
    let mut c = toplevel(6);
    c.req(TOPLEVEL, TOPLEVEL_DESTROY, &[]);
    c.req(XDG_SURFACE, XDG_SURFACE_DESTROY, &[]);
    c.req(WM_BASE, GET_XDG_SURFACE, &[A::U(SECOND), A::U(SURFACE)]);
    c.req(SECOND, GET_TOPLEVEL, &[A::U(61)]);
    assert_eq!(error(&mut c), None);
    let s = c.s.guest.scene.surfaces.iter().find(|s| s.id == SURFACE).expect("the surface");
    assert_eq!((s.xdg, s.toplevel), (Some(SECOND), Some(61)));
}
