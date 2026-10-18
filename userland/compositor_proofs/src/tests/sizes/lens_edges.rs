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

//! The lens and the pointer's place on a screen, every input at an edge:
//! the same as the test's reading in u128, held to the types they land in.

use super::Pick;
use crate::display::cursor_rule::{on_head, on_screen};
use crate::display::magnifier_rule::lens;
use crate::sw_blitter::fit::fit_box;

#[test]
fn the_lens_agrees_with_u128_at_every_edge() {
    let mut p = Pick(0x7000_1E25);
    for _ in 0..200_000 {
        let (at, view, screen, m) =
            ((p.edge(), p.edge()), (p.edge(), p.edge()), (p.edge(), p.edge()), p.edge());
        let z = (screen.0 as u128 * m as u128 / view.0.max(1) as u128).max(1);
        let seen = |s: u32| ((s as u128 * 4).div_ceil(z)).min(u32::MAX as u128) as u32;
        let origin = |a: u32, s: u32, v: u32| a.saturating_sub(s / 2).min(v.saturating_sub(s));
        let l = lens(at, view, screen, m);
        assert_eq!(l.z as u128, z.min(u32::MAX as u128), "{at:?} {view:?} {screen:?} {m}");
        assert_eq!(
            (l.ox, l.oy),
            (origin(at.0, seen(screen.0), view.0), origin(at.1, seen(screen.1), view.1))
        );
    }
}

#[test]
fn the_pointer_lands_as_u128_says_at_every_edge() {
    let mut p = Pick(0x7000_C025);
    let cap = |v: u128| v.min(i32::MAX as u128) as i32;
    for _ in 0..200_000 {
        let (at, canvas, screen, q) =
            ((p.edge(), p.edge()), (p.edge(), p.edge()), (p.edge(), p.edge()), p.edge());
        let shrunk = canvas.0 > screen.0 || canvas.1 > screen.1;
        let f = |v: u32, c: u32, s: u32| match shrunk {
            true => cap(v as u128 * s as u128 / c.max(1) as u128),
            false => cap(v as u128 * q.max(4) as u128 / 4),
        };
        let want = (f(at.0, canvas.0, screen.0), f(at.1, canvas.1, screen.1));
        assert_eq!(on_screen(at, canvas, screen, q), want);
        let pt = (p.edge() as i32, p.edge() as i32);
        let (ox, oy, bw, bh) = fit_box(screen, canvas);
        let g = |v: i32, s: u32, b: u32, o: u32| {
            cap(o as u128 + v.max(0) as u128 * b as u128 / s.max(1) as u128)
        };
        assert_eq!(
            on_head(pt, screen, canvas),
            (g(pt.0, screen.0, bw, ox), g(pt.1, screen.1, bh, oy))
        );
    }
}
