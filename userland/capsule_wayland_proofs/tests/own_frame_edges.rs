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
//! A guest window resized from its edges (toplevel/resize.rs): held by a
//! A window that draws its own frame (decoration.rs, no decoration made)
//! is cut to its window geometry, so its client's own resize margin, its
//! shadow, takes no press: live, Telegram pressed 3 px inside its right edge
//! and dragged 150 px stayed 1000x750. The band just inside its edge now
//! resizes it as a framed window's border does, and a press past the band
//! is the client's.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input::{motion, post, Kind};

const POINTER: u32 = 14;
const DECO_MANAGER: u32 = 30;
const CONFIGURE: u16 = 0;
const POINTER_BUTTON: u16 = 3;
const BLUE: u32 = 0xFF20_40A0;

/// A shown 400 by 300 window whose client bound the decoration manager and
/// made no decoration, so draws its own frame, with a pointer.
fn own_frame() -> Client {
    let mut c = toplevel(6);
    c.bind(40, &[("zxdg_decoration_manager_v1", 1, DECO_MANAGER)]);
    c.events();
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (400, 300), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    assert!(c.s.guest.scene.win.frame.own_frame, "the client draws its own frame");
    c
}

fn on_screen() -> (u32, u32, u32, u32) {
    desk::with(|d| {
        let l = d.layers.last().expect("a layer");
        (l.x, l.y, l.width, l.height)
    })
}

fn drag(c: &mut Client, from: (i32, i32), to_: (i32, i32)) -> Vec<Ev> {
    motion(from.0, from.1);
    c.events();
    post(Kind::ButtonDown, 0, 1, from, (0, 0));
    motion(to_.0, to_.1);
    post(Kind::ButtonUp, 0, 1, to_, (0, 0));
    c.events()
}

fn told(evs: &[Ev]) -> Vec<(u32, u32)> {
    let configures = to(evs, TOPLEVEL).into_iter().filter(|e| e.opcode == CONFIGURE);
    configures.map(|e| (e.word(0), e.word(1))).collect()
}

fn pressed(evs: &[Ev]) -> bool {
    to(evs, POINTER).iter().any(|e| e.opcode == POINTER_BUTTON)
}

#[test]
fn a_press_just_inside_its_right_edge_resizes_it() {
    let mut c = own_frame();
    let (_, _, w, h) = on_screen();
    assert_eq!((w, h), (400, 300), "no NØNOS frame round it");
    let edge = (w as i32 - 2, h as i32 / 2);
    let evs = drag(&mut c, edge, (edge.0 + 50, edge.1));
    assert_eq!(told(&evs), vec![(450, 300)], "told once, at the release");
    assert!(!pressed(&evs), "the press was the resize's, not the client's");
}

#[test]
fn its_corner_resizes_both_ways() {
    let mut c = own_frame();
    let (_, _, w, h) = on_screen();
    let corner = (w as i32 - 1, h as i32 - 1);
    let evs = drag(&mut c, corner, (corner.0 + 40, corner.1 + 30));
    assert_eq!(told(&evs), vec![(440, 330)]);
}

#[test]
fn a_press_past_the_band_is_the_clients() {
    let mut c = own_frame();
    let (_, _, w, h) = on_screen();
    let inside = (w as i32 - 12, h as i32 / 2);
    let evs = drag(&mut c, inside, (inside.0 + 50, inside.1));
    assert!(told(&evs).is_empty(), "no resize");
    assert!(pressed(&evs), "the client took the press");
}
