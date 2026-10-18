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

//! Coverage through a full-screen mark, a zero-sized rectangle and a long
//! cursor sweep: the accumulator still hands back every damaged pixel.

use super::damage_cover::*;
use crate::damage::{DamageAccumulator, Rect};
use std::collections::HashSet;

#[test]
fn mark_full_then_accumulate_covers_the_screen() {
    let mut acc = DamageAccumulator::new();
    acc.mark_full(64, 48);
    acc.accumulate(Rect { x: 10, y: 10, width: 4, height: 4 });
    let mut covered: HashSet<(u32, u32)> = HashSet::new();
    while let Some(out) = acc.drain() {
        covered.extend(cells(out));
    }
    for y in 0..48 {
        for x in 0..64 {
            assert!(covered.contains(&(x, y)), "screen cell {x},{y} not covered");
        }
    }
}

#[test]
fn zero_sized_rects_are_ignored() {
    let mut acc = DamageAccumulator::new();
    acc.accumulate(Rect { x: 5, y: 5, width: 0, height: 10 });
    acc.accumulate(Rect { x: 5, y: 5, width: 10, height: 0 });
    assert!(acc.drain().is_none(), "a zero-area rect must not become work");
}

#[test]
fn many_cursor_moves_never_drop_the_old_position() {
    // Simulate a cursor sweeping across the screen: each step damages the old
    // and new 32x32 box. Every box must end up covered even as merges happen.
    let mut inputs = Vec::new();
    let (mut px, mut py) = (100u32, 100u32);
    for i in 0..20u32 {
        let (nx, ny) = (100 + i * 17, 100 + i * 11);
        inputs.push(Rect { x: px, y: py, width: 32, height: 32 });
        inputs.push(Rect { x: nx, y: ny, width: 32, height: 32 });
        px = nx;
        py = ny;
    }
    assert_covers(&inputs);
}
