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

//! The registry index (`abi::index`) finds every registered tag at its
//! registry slot, and nothing else, in a short probe.

use super::abi::index::{entry, longest_probe, slot_of, COUNT};
use super::abi::REGISTRY;

#[test]
fn the_index_finds_every_entry_and_nothing_else() {
    let mut seen = 0;
    for slice in REGISTRY {
        for e in *slice {
            let slot = slot_of(e.id).unwrap_or_else(|| panic!("{} has no slot", e.name));
            assert_eq!(entry(slot).map(|x| x.id), Some(e.id));
            seen += 1;
        }
    }
    assert_eq!(seen, COUNT);
    let ids: Vec<u64> = REGISTRY.iter().flat_map(|s| s.iter().map(|e| e.id)).collect();
    let mut x = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..200_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        for probe in [x, x & 0xffff_ffff, (x & 0x7f7f_7f7f) | 0x4040_4040] {
            assert_eq!(slot_of(probe).is_some(), ids.contains(&probe), "{probe:#x}");
        }
    }
    assert!(slot_of(0).is_none() && slot_of(u64::MAX).is_none());
}

#[test]
fn the_index_slots_are_registry_order() {
    let mut n = 0;
    for slice in REGISTRY {
        for e in *slice {
            assert_eq!(entry(n).map(|x| x.id), Some(e.id));
            n += 1;
        }
    }
    assert!(entry(n).is_none());
}

#[test]
fn every_tag_is_found_within_four_probes() {
    assert!(longest_probe() <= 4, "longest probe {}", longest_probe());
}
