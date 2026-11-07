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

//! A window whose client draws its own shadow round it, as GTK's and
//! Telegram's do, names its window geometry inside the buffer. Shown from
//! its pin, the whole buffer is pinned and the compositor draws only the
//! geometry's box of it (Content view), with what the client said changed
//! told in that box's pixels: nothing read out of the guest, and on screen
//! the same pixels the copy shows.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::linux::call::set_clock;
use nonos_capsule_wayland_proofs::linux::guest_reads;

const W: i32 = 320;
const H: i32 = 200;
/// The window inside the shadow: xdg_surface.set_window_geometry.
const G: (i32, i32, i32, i32) = (10, 8, 300, 184);
const SET_WINDOW_GEOMETRY: u16 = 3;
const PX: u32 = 0xFF20_40A0;

fn pins(allow: bool) {
    nonos_libc::world::with(|w| w.allow_pins = allow);
}

/// A framed toplevel with its geometry set, shown twice whole from 101,
/// with 101 and 103 painted.
fn window() -> Client {
    set_clock(1_000);
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("shadowed")]);
    c.req(XDG_SURFACE, SET_WINDOW_GEOMETRY, &[A::I(G.0), A::I(G.1), A::I(G.2), A::I(G.3)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    for n in 0..2 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (W, H), PX);
    }
    commit(&mut c, 101, (0, 0, W, H));
    commit(&mut c, 101, (0, 0, W, H));
    c
}

fn commit(c: &mut Client, buffer: u32, b: (i32, i32, i32, i32)) {
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

type Told = (u64, (u32, u32), (u32, u32, u32, u32), (u32, u32, u32, u32));

/// Each content named: its handle, corner, box changed and box drawn.
fn told() -> Vec<Told> {
    let calls = desk::with(|d| d.calls.clone());
    calls
        .iter()
        .filter_map(|c| match c {
            Call::Content(_, handle, at, changed, view) => Some((*handle, *at, *changed, *view)),
            _ => None,
        })
        .collect()
}

#[test]
fn a_window_cut_to_its_geometry_is_shown_from_its_pin_as_that_box() {
    pins(true);
    let mut c = window();
    desk::with(|d| d.calls.clear());
    guest_reads::take();
    commit(&mut c, 103, (0, 0, W, H));
    commit(&mut c, 101, (30, 20, 40, 10));
    let r = guest_reads::take();
    assert_eq!((r.calls, r.bytes), (0, 0), "nothing read out of the guest");
    let t = told();
    assert_eq!(t.len(), 2, "both commits named their pins: {t:?}");
    let view = (G.0 as u32, G.1 as u32, G.2 as u32, G.3 as u32);
    assert!(t.iter().all(|x| x.0 != 0 && x.3 == view), "the geometry's box drawn: {t:?}");
    assert_eq!(t[0].2, (0, 0, G.2 as u32, G.3 as u32), "the first pin all of the box");
    assert_eq!(t[1].2, (20, 12, 40, 10), "the box said, in the drawn box's pixels");
    let pinned = nonos_libc::world::with(|w| w.pinned);
    assert_eq!(pinned, 2, "the whole buffer pinned, one pin a buffer");
}

/// The window's pixels with 103 committed, and where the pinned content
/// is drawn over them by the compositor, painted as it would be.
fn screen(pinned: bool) -> (u32, Vec<u32>) {
    pins(pinned);
    let mut c = window();
    desk::with(|d| d.calls.clear());
    commit(&mut c, 103, (0, 0, W, H));
    let (w, h, mut px) = nonos_capsule_wayland_proofs::host::dump::pixels(&c.s.guest).unwrap();
    let named = told().last().copied();
    assert_eq!(named.is_some(), pinned, "content named exactly when pinned");
    if let Some((_, (cx, cy), _, (_, _, vw, vh))) = named {
        for y in cy..(cy + vh).min(h) {
            for x in cx..(cx + vw).min(w) {
                px[(y * w + x) as usize] = PX;
            }
        }
    }
    (w, px)
}

#[test]
fn a_pinned_window_cut_to_its_geometry_is_the_copied_one_pixel_for_pixel() {
    let (w, copied) = screen(false);
    let (pw, pinned) = screen(true);
    assert_eq!(w, pw);
    if let Some(i) = (0..copied.len()).find(|&i| copied[i] != pinned[i]) {
        panic!(
            "({}, {}): copied {:#010x}, pinned {:#010x}",
            i as u32 % w,
            i as u32 / w,
            copied[i],
            pinned[i]
        );
    }
}
