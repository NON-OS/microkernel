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

//! The idle levels a MkAcpiCpu record carries: the processor's own first,
//! containers above it, every parent a state the level above has.

use super::cpu_layout::*;
use super::cpu_wire::decode;
use super::lpi_build::arm_core;

#[test]
fn the_levels_decode_nearest_first() {
    let t = decode(&arm_core(3)).expect("well formed");
    let c: Vec<Option<u64>> = t.levels.iter().map(|l| l.container).collect();
    assert_eq!(c, [None, Some(0x10), Some(0x100)]);
    let own = &t.levels[0].states;
    assert_eq!(
        (own[1].entry.space, own[1].entry.value, own[1].parent_state),
        (0x7F, 0x4000_0003, 2)
    );
    assert_eq!((own[1].residency.value, own[1].usage.kind), (0x9000, ABSENT));
    let cluster = &t.levels[1];
    assert_eq!((cluster.level_id, cluster.states[1].entry.kind), (1, INTEGER));
    assert_eq!(cluster.states[1].wake_latency_us, 300);
}

/// Where in the record the first level starts.
const AT: usize = HEADER;

#[test]
fn levels_are_checked() {
    let b = arm_core(3);
    let mut own = b.clone();
    own[AT + 16] = LPI_CONTAINER;
    assert_eq!(decode(&own), None, "level 0 is the processor's own");
    let mut named = b.clone();
    named[AT] = 1;
    assert_eq!(decode(&named), None, "the processor's own level names no container");
    let mut parent = b.clone();
    parent[AT + LEVEL + LPI_STATE + 12] = 3;
    assert_eq!(decode(&parent), None, "the cluster has two states, not three");
    let mut top = b.clone();
    let last = b.len() - LPI_STATE + 12;
    top[last] = 9;
    assert!(decode(&top).is_some(), "the top level's parent is beyond what was read");
    let mut count = b.clone();
    count[4] = MAX_LEVELS as u8 + 1;
    assert_eq!(decode(&count), None);
    assert_eq!(decode(&b[..b.len() - 1]), None, "cut short");
}
