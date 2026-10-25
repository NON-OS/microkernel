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

//! The table against an unbounded model over random calls, more pairs than
//! slots: every report the model makes, the table has made too, in the same
//! silence and no later. Eviction makes reports early, never drops one.

use super::silence::{Pair, Silence, SLOTS};
use std::collections::BTreeMap;

const W: u64 = 5000;

#[test]
fn the_table_never_reports_later_than_an_unbounded_model() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut next = |m: u64| {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) % m
    };
    let mut table = Silence::new(W);
    let mut since: BTreeMap<u32, u64> = BTreeMap::new();
    let mut told: BTreeMap<u32, u64> = BTreeMap::new();
    let mut now = 0u64;
    for _ in 0..200_000 {
        now += next(300);
        let caller = next(SLOTS as u64 + 12) as u32;
        let pair = Pair { caller, port: 1 };
        if next(10) == 0 {
            table.answered(pair);
            since.remove(&caller);
            continue;
        }
        let start = now.saturating_sub(next(100));
        for r in table.late(pair, start, now, now + 1).into_iter().flatten() {
            told.insert(r.caller, now);
        }
        let s = *since.entry(caller).or_insert(start);
        if now - s >= W {
            let at = told.get(&caller).copied().unwrap_or(0);
            assert!(at >= s && at <= now, "pair {caller} silent since {s} not told by {now}");
            since.insert(caller, now);
        }
    }
}
