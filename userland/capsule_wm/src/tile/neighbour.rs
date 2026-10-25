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
//! The window focus moves to from `from` in a direction: of the windows
//! wholly beyond its edge that way, the nearest, and of those the one most
//! in line with it.

use super::types::{Dir, Win};
use crate::geometry::rect::Rect;

pub fn neighbour(placed: &[(Win, Rect)], from: Win, dir: Dir) -> Option<Win> {
    let f = placed.iter().find(|p| p.0 == from)?.1;
    placed
        .iter()
        .filter(|(w, r)| *w != from && beyond(f, *r, dir))
        .min_by_key(|(_, r)| (distance(f, *r, dir), off_line(f, *r, dir)))
        .map(|p| p.0)
}

fn end(r: Rect) -> (u32, u32) {
    (r.x.saturating_add(r.width), r.y.saturating_add(r.height))
}

fn beyond(f: Rect, r: Rect, dir: Dir) -> bool {
    match dir {
        Dir::Right => r.x >= end(f).0,
        Dir::Left => end(r).0 <= f.x,
        Dir::Down => r.y >= end(f).1,
        Dir::Up => end(r).1 <= f.y,
    }
}

fn distance(f: Rect, r: Rect, dir: Dir) -> u32 {
    match dir {
        Dir::Right => r.x - end(f).0,
        Dir::Left => f.x - end(r).0,
        Dir::Down => r.y - end(f).1,
        Dir::Up => f.y - end(r).1,
    }
}

fn off_line(f: Rect, r: Rect, dir: Dir) -> u32 {
    let mid = |at: u32, len: u32| u64::from(at) * 2 + u64::from(len);
    let (a, b) = match dir {
        Dir::Left | Dir::Right => (mid(f.y, f.height), mid(r.y, r.height)),
        Dir::Up | Dir::Down => (mid(f.x, f.width), mid(r.x, r.width)),
    };
    a.abs_diff(b).min(u64::from(u32::MAX)) as u32
}
