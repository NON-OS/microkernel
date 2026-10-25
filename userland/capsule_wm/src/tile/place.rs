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
//! Where each tiled window goes in the work area: the outer gap off every
//! edge, then each split shared equally between its children with the inner
//! gap between them; the pixels that do not divide go one each to the first
//! children, so the whole length is used.

use alloc::vec::Vec;

use super::node::Node;
use super::types::{Gaps, Split, Win};
use crate::geometry::rect::Rect;

pub fn layout(tree: &Node, work: Rect, g: Gaps) -> Vec<(Win, Rect)> {
    let o = g.outer;
    let area = Rect {
        x: work.x.saturating_add(o),
        y: work.y.saturating_add(o),
        width: work.width.saturating_sub(o.saturating_mul(2)),
        height: work.height.saturating_sub(o.saturating_mul(2)),
    };
    let mut out = Vec::new();
    place(tree, area, g.inner, &mut out);
    out
}

fn place(node: &Node, a: Rect, gap: u32, out: &mut Vec<(Win, Rect)>) {
    let (split, kids) = match node {
        Node::Leaf(w) => return out.push((*w, a)),
        Node::Split(s, kids) => (*s, kids),
    };
    let n = kids.len() as u32;
    let along = if split == Split::Across { a.width } else { a.height };
    let room = along.saturating_sub(gap.saturating_mul(n.saturating_sub(1)));
    let mut at = 0u32;
    for (i, k) in kids.iter().enumerate() {
        let size = room / n + u32::from((i as u32) < room % n);
        let r = match split {
            Split::Across => Rect { x: a.x + at, y: a.y, width: size, height: a.height },
            Split::Down => Rect { x: a.x, y: a.y + at, width: a.width, height: size },
        };
        place(k, r, gap, out);
        at = at.saturating_add(size).saturating_add(gap);
    }
}
