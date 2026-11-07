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

//! What any boxes a client names must leave (wayland/damage_boxes.rs), at
//! the whole range of the i32s wl_surface.damage_buffer carries: never more
//! than MAX_BOXES, none empty, each inside the one box round all named
//! (damage.rs), and each box named whole inside one kept box. Moved and cut
//! to any box a commit shows, each is inside it and not empty, and apart
//! answers only for two or more. The fuzz target holds every input to this.

use crate::damage::{corners, Damage};
use crate::damage_boxes::{Boxes, Corners, MAX_BOXES};

/// The most boxes one input names.
const MOST: usize = 16;

fn word(b: &[u8]) -> u32 {
    let mut w = [0u8; 4];
    w[..b.len()].copy_from_slice(b);
    u32::from_le_bytes(w)
}

fn inside((x0, y0, x1, y1): Corners, (a0, b0, a1, b1): Corners) -> bool {
    x0 >= a0 && y0 >= b0 && x1 <= a1 && y1 <= b1
}

fn sound(b: &Boxes) {
    assert!(b.count() <= MAX_BOXES, "{} boxes kept", b.count());
    assert_eq!(b.iter().count(), b.count());
    for (x0, y0, x1, y1) in b.iter() {
        assert!(x0 < x1 && y0 < y1, "an empty box kept: {:?}", (x0, y0, x1, y1));
    }
    let (areas, n) = b.areas();
    for ((x, y, w, h), (x0, y0, x1, y1)) in areas[..n].iter().zip(b.iter()) {
        assert_eq!((*x, *y, *w, *h), (x0, y0, x1 - x0, y1 - y0));
    }
}

/// The boxes `named` at `scale`, then cut from (`dx`, `dy`) to `bound`.
pub fn check(named: &[(i32, i32, i32, i32)], scale: u32, cut: (u32, u32), bound: Corners) {
    let (mut b, mut d) = (Boxes::default(), Damage::None);
    for &n in named {
        b = b.add(n, scale);
        d = d.add(n, scale);
        sound(&b);
    }
    let round = match d {
        Damage::Rect(x0, y0, x1, y1) => Some((x0, y0, x1, y1)),
        _ => None,
    };
    match round {
        None => assert_eq!(b.count(), 0, "boxes kept with nothing named"),
        Some(r) => assert!(b.iter().all(|k| inside(k, r)), "a box past {r:?}"),
    }
    for c in named.iter().filter_map(|&n| corners(n, scale)) {
        assert!(b.iter().any(|k| inside(c, k)), "{c:?} is in no box kept");
    }
    let w = b.within(cut, bound);
    sound(&w);
    assert!(w.iter().all(|k| inside(k, bound)), "a box past the one shown, {bound:?}");
    assert_eq!(b.apart(cut, bound), (w.count() >= 2).then_some(w));
}

/// Any bytes as a fuzz input: the scale, the cut and the box shown in the
/// first 28 bytes, then up to MOST boxes of four words each.
pub fn check_bytes(data: &[u8]) {
    let w = |i: usize| word(data.get(i * 4..data.len().min(i * 4 + 4)).unwrap_or(&[]));
    let scale = w(0) % 5;
    let cut = (w(1), w(2));
    let bound = (w(3), w(4), w(5), w(6));
    let rest = data.get(28..).unwrap_or(&[]);
    let named: Vec<_> = rest
        .chunks_exact(16)
        .take(MOST)
        .map(|c| {
            let i = |k: usize| word(&c[k * 4..k * 4 + 4]) as i32;
            (i(0), i(1), i(2), i(3))
        })
        .collect();
    check(&named, scale, cut, bound);
}
