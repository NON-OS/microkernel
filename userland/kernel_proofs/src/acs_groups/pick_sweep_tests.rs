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

//! The pick over seeded trees (L77, L77c): root ports, switches nested
//! three deep, ports reporting anything, bridges firmware left unset. Each
//! counted port is drawn closed or open, as `close` may find it, and the
//! groups are built from them: two ports of one switch are both closed, or
//! the buses below them share a group.

use super::acs_span::{group_of, groups, Port};
use super::pick_gen::ROOT;
use super::pick_trees::{grown, last_below};
use super::port_rule::{picked, DOWNSTREAM, UPSTREAM};

#[test]
fn two_ports_of_a_switch_are_both_closed_or_grouped_together() {
    let mut seen = [0u32; 3];
    for seed in 1..=4_000u64 {
        let mut t = grown(seed, true);
        let bridges = t.0.clone();
        let pick = picked(&bridges);
        let mut ports = Vec::new();
        let mut shut = vec![false; t.0.len()];
        for at in (0..t.0.len()).filter(|&at| pick(at)) {
            shut[at] = t.draw(4) != 0;
            let below = t.0[at].1.map(|first| (first, last_below(&t, at)));
            ports.push(Port { bus: t.0[at].0, below, closed: shut[at] });
        }
        let g = groups(&ports);
        let switch = |at: usize| (t.1[at].0 == DOWNSTREAM).then(|| t.1[at].1);
        let group = |at: usize| t.0[at].1.and_then(|b| group_of(&g, b));
        for a in 0..t.0.len() {
            assert!(!pick(a) || t.1[a].0 != ROOT, "seed {seed}: root port {a} in a switch");
            for b in (a + 1..t.0.len()).filter(|&b| switch(a).is_some() && switch(b) == switch(a)) {
                let together = group(a).is_some() && group(a) == group(b);
                assert!(together || (shut[a] && shut[b]), "seed {seed}: ports {a}, {b} reach");
                seen[together as usize] += 1;
            }
            seen[2] += (switch(a).is_some() && t.0[t.1[a].1].2 != Some(UPSTREAM)) as u32;
        }
    }
    assert!(seen[0] > 2_000 && seen[1] > 2_000 && seen[2] > 2_000, "{seen:?}");
}
