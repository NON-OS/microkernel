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

//! A window's pixels are pages of their own, unmapped whole once their
//! surface is released, so a later buffer is never given pages a surface
//! still viewed holds: live, the restored Telegram's surface was refused,
//! `surface 1045x795 refused`, its pixels being the heap's reused pages of
//! the maximised pair, and the window stayed drawn at its maximised size.

use nonos_capsule_wayland_proofs::client::*;
use nonos_libc::world;

const POINTER: u32 = 14;
const CONFIGURE: u16 = 0;
const SET_MAXIMIZED: u16 = 9;
const UNSET_MAXIMIZED: u16 = 10;
const BLUE: u32 = 0xFF20_40A0;

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

fn told(evs: &[Ev]) -> (i32, i32) {
    let configures = to(evs, TOPLEVEL).into_iter().filter(|e| e.opcode == CONFIGURE);
    let last = configures.last().expect("a configure");
    (last.word(0) as i32, last.word(1) as i32)
}

/// The last configure in `evs` acked and a buffer of `size` drawn for it.
fn draw(c: &mut Client, evs: &[Ev], (pool, buffer): (u32, u32), size: (i32, i32)) {
    let serial = to(evs, XDG_SURFACE).last().expect("a configure").word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, pool, buffer, size, BLUE);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

/// Every surface registered now whose pixels are a mapping made for it.
fn surfaces_on_mappings() -> (usize, usize) {
    world::with(|w| {
        let live: Vec<_> = w.surfaces.iter().flatten().collect();
        let mapped = |d: &&&world::SurfaceDescriptor| {
            w.maps.iter().any(|&(at, len)| (d.base_va, d.byte_len) == (at as u64, len as u64))
        };
        (live.len(), live.iter().filter(mapped).count())
    })
}

#[test]
fn a_window_resized_back_and_forth_unmaps_each_old_pair_and_never_one_still_registered() {
    let mut c = window();
    let (n, on) = surfaces_on_mappings();
    assert!(n > 0 && on == n, "each buffer its own mapping, {on} of {n}");
    let first = world::with(|w| w.maps.len());
    for round in 0..3u32 {
        let pools = (60 + 4 * round, 61 + 4 * round);
        c.req(TOPLEVEL, SET_MAXIMIZED, &[]);
        let evs = c.events();
        draw(&mut c, &evs, pools, told(&evs));
        c.req(TOPLEVEL, UNSET_MAXIMIZED, &[]);
        let evs = c.events();
        draw(&mut c, &evs, (pools.0 + 2, pools.1 + 2), told(&evs));
    }
    let (n, on) = surfaces_on_mappings();
    assert_eq!(on, n, "every buffer shown is still its own mapping");
    world::with(|w| {
        assert_eq!(w.unmapped_registered, 0, "no pages a registered surface names were unmapped");
        assert!(w.unmaps.len() >= 6, "each old pair given back, {} unmaps", w.unmaps.len());
        assert!(w.maps.len() <= first + 2, "{} mappings held, {first} at the start", w.maps.len());
    });
}
