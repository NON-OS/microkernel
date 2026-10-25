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
//! Placing a tree in the work area: the outer gap off every edge, children
//! sharing their split equally with the inner gap between them, the whole
//! length used, nothing overlapping and nothing outside.

use crate::rect::Rect;
use crate::tile::node::Node::{Leaf, Split as S};
use crate::tile::place::layout;
use crate::tile::types::Gaps;
use crate::tile::types::Split::{Across, Down};

const WORK: Rect = Rect { x: 0, y: 46, width: 1920, height: 954 };

fn r(x: u32, y: u32, width: u32, height: u32) -> Rect {
    Rect { x, y, width, height }
}

fn q(a: Rect) -> (u32, u32, u32, u32) {
    (a.x, a.y, a.width, a.height)
}

#[test]
fn one_window_fills_the_area_inside_the_outer_gap() {
    let p = layout(&Leaf((1, 1)), WORK, Gaps { inner: 6, outer: 10 });
    assert_eq!(p.len(), 1);
    assert_eq!((p[0].0, q(p[0].1)), ((1, 1), (10, 56, 1900, 934)));
}

#[test]
fn a_split_shares_its_length_with_the_gap_between() {
    let t = S(Across, vec![Leaf((1, 1)), Leaf((2, 1)), Leaf((3, 1))]);
    let p = layout(&t, r(0, 0, 1000, 500), Gaps { inner: 10, outer: 0 });
    // 1000 - 2 gaps = 980: 327, 327, 326, the spare pixels to the first.
    assert_eq!(q(p[0].1), (0, 0, 327, 500));
    assert_eq!(q(p[1].1), (337, 0, 327, 500));
    assert_eq!(q(p[2].1), (674, 0, 326, 500));
}

#[test]
fn nested_splits_tile_the_area_exactly() {
    let t = S(Across, vec![Leaf((1, 1)), S(Down, vec![Leaf((2, 1)), Leaf((3, 1))])]);
    for (w, h, inner, outer) in [(1920, 954, 8, 12), (1280, 754, 0, 0), (801, 599, 3, 5)] {
        let p = layout(&t, r(0, 0, w, h), Gaps { inner, outer });
        let area: u64 = p.iter().map(|(_, q)| u64::from(q.width) * u64::from(q.height)).sum();
        let (iw, ih) = (u64::from(w - 2 * outer), u64::from(h - 2 * outer));
        let right = (iw - u64::from(inner)) / 2;
        assert_eq!(area, iw * ih - u64::from(inner) * ih - u64::from(inner) * right);
        for (i, (_, a)) in p.iter().enumerate() {
            assert!(a.x >= outer && a.y >= outer, "{w}x{h}: inside");
            assert!(a.x + a.width <= w - outer && a.y + a.height <= h - outer, "{w}x{h}");
            for (_, b) in &p[i + 1..] {
                let apart = a.x + a.width <= b.x || b.x + b.width <= a.x;
                assert!(apart || a.y + a.height <= b.y || b.y + b.height <= a.y, "{w}x{h}");
            }
        }
    }
}
