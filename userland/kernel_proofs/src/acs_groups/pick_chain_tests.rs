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

//! The chain above a bridge (L77c). A switch whose ports all lie counts
//! its downstream ports below an upstream port nothing above bears out, so
//! they are closed or grouped even when that upstream port closes itself;
//! a fully swapped switch nested below a lying one is found the same way.
//! Told the truth, the pick counts the switch ports and nothing else.

use super::acs_span::{group_of, groups, Port};
use super::pick_gen::ROOT;
use super::pick_trees::grown;
use super::port_rule::{picked, Bridge, DOWNSTREAM, UPSTREAM};

fn counted(b: &[Bridge]) -> Vec<usize> {
    let pick = picked(b);
    (0..b.len()).filter(|&at| pick(at)).collect()
}

#[test]
fn a_swapped_switch_is_closed_or_grouped_below_its_closed_upstream_port() {
    for u in [DOWNSTREAM, ROOT, 0, 7] {
        let tree = [(0, Some(1), Some(ROOT)), (1, Some(2), Some(u))];
        let tree = [tree[0], tree[1], (2, Some(3), Some(UPSTREAM)), (2, Some(4), Some(UPSTREAM))];
        assert_eq!(counted(&tree), vec![1, 2, 3], "its upstream port says {u}");
        let port = |bus, first, last, closed| Port { bus, below: Some((first, last)), closed };
        let g = groups(&[port(1, 2, 4, true), port(2, 3, 3, false), port(2, 4, 4, false)]);
        assert!(group_of(&g, 3).is_some() && group_of(&g, 3) == group_of(&g, 4), "{g:?}");
    }
    let nested = [
        (0, Some(1), Some(ROOT)),
        (1, Some(2), Some(UPSTREAM)),
        (2, Some(3), Some(UPSTREAM)),
        (3, Some(4), Some(DOWNSTREAM)),
        (4, Some(5), Some(UPSTREAM)),
        (4, Some(6), Some(UPSTREAM)),
    ];
    assert_eq!(counted(&nested), vec![2, 3, 4, 5], "a swapped switch below a lying port");
    let mut honest = nested;
    (honest[2].2, honest[3].2, honest[4].2, honest[5].2) =
        (Some(DOWNSTREAM), Some(UPSTREAM), Some(DOWNSTREAM), Some(DOWNSTREAM));
    assert_eq!(counted(&honest), vec![2, 4, 5], "the same switches, honest");
}

#[test]
fn told_the_truth_the_pick_counts_switch_ports_and_unset_bridges_only() {
    for seed in 1..=4_000u64 {
        let t = grown(seed, false);
        // One firmware left unset is read with no type: below a bridge it counts.
        let said = |(b, k): (&Bridge, &(u8, usize))| (b.0, b.1, Some(k.0).filter(|&k| k != 0));
        let told: Vec<Bridge> = t.0.iter().zip(&t.1).map(said).collect();
        let pick = picked(&told);
        for (at, &(kind, _)) in t.1.iter().enumerate() {
            assert_eq!(
                pick(at),
                kind == DOWNSTREAM || kind == 0,
                "seed {seed}: bridge {at} {told:?} {:?}",
                t.1
            );
        }
    }
}
