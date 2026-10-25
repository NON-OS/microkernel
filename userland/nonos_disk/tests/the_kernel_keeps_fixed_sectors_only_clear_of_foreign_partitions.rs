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

//! The fixed sectors over seeded tables: refused when another system's
//! partition crosses the fixed store or plan, at any sector up to their
//! edges; kept for an older NONOS disk whose table names an ESP clear of
//! them, for a disk with no GPT and for one whose two headers failed;
//! moved when the table names NONOS's own.

extern crate alloc;

#[path = "common/fixed_sweep.rs"]
mod fixed_sweep;
#[path = "common/fixed_tables.rs"]
mod fixed_tables;
#[path = "common/kernel_gpt/mod.rs"]
#[allow(dead_code, unused_imports)]
mod kernel_gpt;

use fixed_sweep::{kernel, rule, EDGES, ESP, FIXED_HI, FIXED_LO};
use kernel_gpt::{layout, Layout, Shown, Table};

#[test]
fn a_foreign_partition_anywhere_over_the_fixed_sectors_refuses_them() {
    let mut seen = [0u32; 6];
    let mut r = fixed_tables::Rng(0x9E37_79B9_7F4A_7C15);
    for seed in 0..60_000u64 {
        let (kind, a, b, entries) = fixed_tables::table(&mut r);
        let want = rule(&entries);
        assert_eq!(kernel(entries.clone()), want, "seed {seed}: {entries:?}");
        let at = |s: u64| s >= FIXED_LO && s < FIXED_HI;
        let crosses = at(a.min(b)) || at(a.max(b)) || (a.min(b) < FIXED_LO && a.max(b) >= FIXED_HI);
        seen[match want {
            None => 0,
            Some((.., true)) => 1,
            Some(_) if crosses => 2,
            _ => 3,
        }] += 1;
        seen[4] += (kind == ESP && !crosses && want.is_some()) as u32;
        seen[5] += (EDGES.contains(&a) || EDGES.contains(&b)) as u32;
    }
    assert!(
        seen.iter().all(|&n| n >= 1_000),
        "refused, NONOS-placed, ours over, clear, ESP, edges: {seen:?}"
    );
}

#[test]
fn no_gpt_and_two_failed_headers_keep_the_fixed_sectors_and_say_which() {
    for (t, shown) in [(Table::NoGpt, Shown::NoGpt), (Table::Unbelieved, Shown::Failed)] {
        let Layout::At { places, named, shown: s, .. } = layout(t) else { panic!("refused") };
        assert_eq!(
            (places.store.first, places.plan.first, named, s),
            (FIXED_LO, 245_760, false, shown)
        );
    }
}
