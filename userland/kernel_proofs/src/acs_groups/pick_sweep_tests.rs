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

//! The pick over seeded trees (L77): root ports, switches nested three deep,
//! ports reporting anything, bridges firmware left unset anywhere. A switch
//! port counts whatever it says while its upstream port says what it is, and
//! is covered by that upstream port, counted itself, when both lie.

use super::pick_gen::Tree;
use super::pick_gen::ROOT;
use super::port_rule::{picked, DOWNSTREAM, UPSTREAM};

#[test]
fn every_switch_port_counts_or_is_covered_and_no_honest_root_port_counts() {
    let mut seen = [0u32; 4];
    for seed in 1..=4_000u64 {
        let mut t = Tree(Vec::new(), Vec::new(), 0, seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        for _ in 0..1 + t.draw(4) {
            let said = [None, Some(ROOT), Some(ROOT)][t.draw(3) as usize];
            t.bridge(0, ROOT, said, usize::MAX, 0);
        }
        for (at, &(kind, up)) in t.1.iter().enumerate() {
            let got = picked(&t.0, at);
            assert!(!got || kind != ROOT, "seed {seed}: root port {at} in a switch");
            if kind == DOWNSTREAM {
                let honest = t.0[up].2 == Some(UPSTREAM);
                assert!(got || (!honest && picked(&t.0, up)), "seed {seed}: port {at} left out");
                seen[honest as usize] += 1;
                seen[2] += (!got) as u32;
            }
            seen[3] += (kind == 0) as u32;
        }
    }
    assert!(seen[1] > 3_000 && seen[0] > 3_000 && seen[2] > 100 && seen[3] > 2_000, "{seen:?}");
}
