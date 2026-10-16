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

//! Coalescing: the rectangles a frame drains never touch one another, so no
//! pixel is composed twice; there are never more than the accumulator holds;
//! every pixel damaged is in one; and none reaches past what was damaged.

use std::collections::HashSet;

use crate::damage::{DamageAccumulator, Rect};
use crate::tests::scene::frame_model::Rng;

fn cells(r: Rect) -> impl Iterator<Item = (u32, u32)> {
    (r.y..r.bottom()).flat_map(move |y| (r.x..r.right()).map(move |x| (x, y)))
}

fn drained(acc: &mut DamageAccumulator) -> Vec<Rect> {
    core::iter::from_fn(|| acc.drain()).collect()
}

#[test]
fn a_bridge_between_two_rectangles_leaves_one() {
    let mut acc = DamageAccumulator::new();
    acc.accumulate(Rect { x: 0, y: 0, width: 8, height: 8 });
    acc.accumulate(Rect { x: 20, y: 0, width: 8, height: 8 });
    // Touches the first; grown by it, it overlaps the second as well.
    acc.accumulate(Rect { x: 6, y: 2, width: 16, height: 2 });
    let out = drained(&mut acc);
    assert_eq!(out.len(), 1, "the grown rectangle folded the second in too");
    assert_eq!((out[0].x, out[0].y, out[0].width, out[0].height), (0, 0, 28, 8));
}

#[test]
fn random_damage_is_coalesced_covered_and_bounded() {
    for seed in 1..=400u64 {
        let mut rng = Rng::new(seed);
        let mut acc = DamageAccumulator::new();
        let mut wanted = HashSet::new();
        let (mut x1, mut y1) = (0, 0);
        for _ in 0..1 + rng.below(40) {
            let (x, y) = (rng.below(90), rng.below(70));
            let r = Rect { x, y, width: rng.below(14), height: rng.below(14) };
            if !r.is_empty() {
                wanted.extend(cells(r));
                (x1, y1) = (x1.max(r.right()), y1.max(r.bottom()));
            }
            acc.accumulate(r);
        }
        let out = drained(&mut acc);
        assert!(out.len() <= 8, "seed {seed}: {} rectangles", out.len());
        for (i, a) in out.iter().enumerate() {
            let inside = a.right() <= x1 && a.bottom() <= y1;
            assert!(!a.is_empty() && inside, "seed {seed}: past the damage");
            for b in &out[i + 1..] {
                assert!(!a.touches(*b), "seed {seed}: two drained rectangles touch");
            }
        }
        let covered: HashSet<_> = out.iter().flat_map(|r| cells(*r)).collect();
        assert!(wanted.is_subset(&covered), "seed {seed}: a damaged pixel was dropped");
    }
}
