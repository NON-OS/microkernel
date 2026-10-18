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

//! Damage rectangles at every edge: areas in u64, unions and overlaps
//! that never wrap, and an accumulator that takes any of them.

use super::Pick;
use crate::state::damage::{DamageAccumulator, Rect};

#[test]
fn damage_at_every_edge_never_wraps() {
    let mut p = Pick(0x7000_DA4A);
    let rect = |p: &mut Pick| Rect { x: p.edge(), y: p.edge(), width: p.edge(), height: p.edge() };
    for _ in 0..2_000 {
        let mut acc = DamageAccumulator::new();
        for _ in 0..40 {
            let (a, b) = (rect(&mut p), rect(&mut p));
            assert_eq!(a.area() as u128, a.width as u128 * a.height as u128);
            let u = a.union(b);
            assert!(u.x <= a.x.min(b.x) && u.right() >= a.right().max(b.right()));
            if let Some(i) = a.intersect(b) {
                assert!(i.width <= a.width && i.height <= a.height);
            }
            acc.accumulate(a);
        }
        let mut n = 0;
        while acc.drain().is_some() {
            n += 1;
        }
        assert!(n <= 8, "{n}");
    }
}
