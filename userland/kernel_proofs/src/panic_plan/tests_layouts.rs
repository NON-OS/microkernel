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

//! The plan over many layouts, and at its limits.

use super::helpers::{exact, run, span};
use super::plan::{plan, Span, MAX_KEEP, MAX_SPANS};

#[test]
fn many_layouts_are_exact() {
    let mut x = 0x2545_f491_4f6c_dd1du64;
    let mut next = |m: u64| {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x % m
    };
    for _ in 0..400 {
        let a = next(0x800);
        let heap = span(a, a + next(0x800));
        let keep: Vec<Span> = (0..next(MAX_KEEP as u64 + 1))
            .map(|_| {
                let s = next(0x1000);
                span(s, s + next(0x300))
            })
            .collect();
        exact(heap, &keep);
    }
}

#[test]
fn more_kept_spans_than_it_can_hold_wipe_nothing() {
    let keep = vec![span(0, 1); MAX_KEEP + 1];
    let mut out = [span(0, 0); MAX_SPANS];
    assert_eq!(plan(span(0x1000, 0x2000), &keep, &mut out), None);
}

#[test]
fn an_empty_heap_plans_nothing() {
    assert!(run(span(0x2000, 0x2000), &[span(0, 0x10)]).is_empty());
    assert!(run(span(0x3000, 0x2000), &[]).is_empty());
}
