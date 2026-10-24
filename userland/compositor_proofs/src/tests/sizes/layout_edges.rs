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

//! The extended layout at every edge: narrow enough to back, its slices
//! side by side, each part of a rectangle inside its slice.

use super::canvas_floor::backing;
use super::Pick;
use crate::display::layout_rule::{local, plan};
use crate::state::damage::Rect;

#[test]
fn an_extended_layout_is_narrow_enough_to_back_at_every_edge() {
    let mut p = Pick(0x7000_1A70);
    for _ in 0..100_000 {
        let heads: Vec<_> = (0..p.edge() % 5).map(|_| (p.edge(), p.edge(), p.edge())).collect();
        let Some(plan) = plan((p.edge(), p.edge()), p.edge(), &heads) else { continue };
        assert!(plan.size.0 <= 16384 && backing(plan.size.0, plan.size.1).is_some());
        let s = plan.slices[..plan.n].iter().map(|s| s.w as u64).sum::<u64>();
        assert_eq!(s, plan.size.0 as u64);
        let r = Rect { x: p.edge(), y: p.edge(), width: p.edge(), height: p.edge() };
        for s in &plan.slices[..plan.n] {
            if let Some(l) = local(r, *s) {
                assert!(l.x as u64 + l.width as u64 <= s.w as u64);
            }
        }
    }
}
