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

//! A control lit or put out under the pointer, or a title set, changes only
//! the title bar, so a framed window presents only the band above its
//! content (present/frame/redraw.rs). The frame is painted whole into the
//! window's buffer all the same, and below the bar it is what it was, so
//! the compositor not composing it again shows the frame as painted. A
//! pointer moving onto a framed Linux window's title bar presented all of
//! the window.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::input::motion;
use nonos_capsule_wayland_proofs::linux::wayland::pump;
use nonos_toolkit::decorations::{hit_test_at, DecorationHit};

const POINTER: u32 = 14;
const BLUE: u32 = 0xFF20_40A0;

/// A shown, framed 400 by 300 window with a pointer.
fn window() -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (400, 300), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// The boxes presented since the calls were last cleared.
fn presented() -> Vec<(u32, u32, u32, u32)> {
    let calls = desk::with(|d| d.calls.clone());
    calls
        .iter()
        .filter_map(|c| match c {
            Call::Present(_, _, b) => Some(*b),
            _ => None,
        })
        .collect()
}

/// The rows above the content: the first row the client's blue is on.
fn bar(c: &Client) -> u32 {
    let (w, _, px) = pixels(&c.s.guest).expect("shown");
    let at = px.iter().position(|&p| p == BLUE).expect("the client's pixels are shown");
    let y = at as u32 / w;
    assert!(y > 0, "a framed window has a bar above its content");
    y
}

fn find(c: &Client, hit: DecorationHit) -> (i32, i32) {
    let (w, h, _) = pixels(&c.s.guest).expect("shown");
    let q = c.s.guest.scene.win.frame.quarters;
    let at = (0..w)
        .flat_map(|x| (0..h.min(64)).map(move |y| (x, y)))
        .find(|&(x, y)| hit_test_at(w, h, false, x, y, q) == hit)
        .expect("the control");
    (at.0 as i32, at.1 as i32)
}

#[test]
fn a_control_lit_and_put_out_presents_the_title_bar_alone() {
    let mut c = window();
    let (w, _, _) = pixels(&c.s.guest).expect("shown");
    let bar = bar(&c);
    for hit in [DecorationHit::CloseButton, DecorationHit::MaximizeButton] {
        desk::with(|d| d.calls.clear());
        let at = find(&c, hit);
        motion(at.0, at.1);
        pump(&mut c.s.guest);
        assert_eq!(c.s.guest.scene.win.frame.hover, hit, "lit");
        assert_eq!(presented(), [(0, 0, w, bar)], "{hit:?}: the bar alone");
    }
    desk::with(|d| d.calls.clear());
    let (_, h, _) = pixels(&c.s.guest).expect("shown");
    motion(200, (h / 2) as i32);
    pump(&mut c.s.guest);
    assert_eq!(c.s.guest.scene.win.frame.hover, DecorationHit::None, "put out");
    assert_eq!(presented(), [(0, 0, w, bar)], "put out: the bar alone");
}

#[test]
fn below_the_bar_a_lit_window_is_what_it_was_and_the_bar_shows_the_light() {
    let mut c = window();
    let bar = bar(&c) as usize;
    // The pointer over the content first, as it is before it reaches the bar.
    motion(200, 200);
    pump(&mut c.s.guest);
    let (w, _, before) = pixels(&c.s.guest).expect("shown");
    let row = w as usize;
    let at = find(&c, DecorationHit::CloseButton);
    motion(at.0, at.1);
    pump(&mut c.s.guest);
    let (_, _, lit) = pixels(&c.s.guest).expect("shown");
    let below = |px: &[u32]| px[bar * row..].to_vec();
    let differ = |a: &[u32], b: &[u32]| a.iter().zip(b).filter(|(a, b)| a != b).count();
    assert_eq!(differ(&below(&lit), &below(&before)), 0, "below the bar, as it was");
    assert_ne!(lit[..bar * row], before[..bar * row], "the bar shows the light");
    motion(200, 200);
    pump(&mut c.s.guest);
    let (_, _, out) = pixels(&c.s.guest).expect("shown");
    assert_eq!(differ(&below(&out), &below(&before)), 0, "put out, below the bar as it was");
    assert_ne!(out[..bar * row], lit[..bar * row], "put out, the light is gone");
}

#[test]
fn a_title_set_presents_the_title_bar_alone() {
    let mut c = window();
    let (w, _, _) = pixels(&c.s.guest).expect("shown");
    let bar = bar(&c);
    desk::with(|d| d.calls.clear());
    c.req(TOPLEVEL, 2, &[A::S("a title of its own")]);
    c.events();
    assert_eq!(presented(), [(0, 0, w, bar)], "the bar alone");
}
