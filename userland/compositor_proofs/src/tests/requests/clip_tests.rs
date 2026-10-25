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

//! A rectangle a client names against the canvas: refused only when it
//! is empty or starts off the canvas, else cut to the canvas edge, for
//! damage and for a placed layer alike; a pointer only on the canvas.

use super::hostile::word;
use super::rng::Rng;
use crate::requests::area::Area;
use crate::requests::place::Placed;
use crate::requests::pointer::Cursor;

/// The test's own cut: the part of `[x, x + w)` by `[y, y + h)` on the
/// canvas, held in u64 so no sum can wrap.
fn cut(a: [u32; 4], canvas: (u32, u32)) -> Option<[u32; 4]> {
    let [x, y, w, h] = a.map(u64::from);
    let (cw, ch) = (canvas.0 as u64, canvas.1 as u64);
    if w == 0 || h == 0 || x >= cw || y >= ch {
        return None;
    }
    Some([x, y, (x + w).min(cw) - x, (y + h).min(ch) - y].map(|v| v as u32))
}

#[test]
fn a_rectangle_is_cut_to_the_canvas_and_refused_only_when_empty_or_off_it() {
    let mut r = Rng::new(0xC11F_0001);
    let (mut cut_to, mut whole, mut refused) = (0usize, 0usize, 0usize);
    for _ in 0..200_000 {
        let a = [word(&mut r), word(&mut r), word(&mut r), word(&mut r)];
        let canvas = (r.range(1, 8192), r.range(1, 8192));
        let want = cut(a, canvas);
        let area = Area { x: a[0], y: a[1], width: a[2], height: a[3] };
        let got = area.clip(canvas.0, canvas.1).map(|c| [c.x, c.y, c.width, c.height]);
        assert_eq!(got, want, "{a:?} on {canvas:?}");
        let placed = Placed { handle: r.next(), x: a[0], y: a[1], width: a[2], height: a[3], z: 7 };
        let layer = placed.layer(42, canvas.0, canvas.1);
        assert_eq!(layer.map(|l| [l.x, l.y, l.width, l.height]), want);
        assert!(layer
            .is_none_or(|l| l.owner_pid == 42 && l.z == 7 && l.surface_handle == placed.handle));
        let c = Cursor { x: a[0], y: a[1], visible: true };
        assert_eq!(c.on(canvas.0, canvas.1), a[0] < canvas.0 && a[1] < canvas.1);
        match want {
            None => refused += 1,
            Some(w) if w[2..] == a[2..] => whole += 1,
            Some(_) => cut_to += 1,
        }
    }
    assert!(cut_to >= 10_000 && whole >= 10_000 && refused >= 10_000, "{cut_to} {whole} {refused}");
}
