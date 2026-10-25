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

use super::acs_span::{group_of, groups};
use super::model::Tree;

// A seeded xorshift, as F-98's sweeps draw.
fn drawn(seed: u64) -> impl FnMut(u64) -> u64 {
    let mut x = seed | 1;
    move |n| {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x % n
    }
}

// Two devices one of which reaches the other past the IOMMU are in one group,
// on 3000 trees; and the groups are not so wide that everything is one.
#[test]
fn every_pair_that_reaches_past_the_iommu_is_one_group() {
    let (mut reached, mut kept_apart) = (0u32, 0u32);
    for seed in 1..=3000u64 {
        let tree = Tree::build(&mut drawn(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15)));
        let g = groups(&tree.ports);
        for x in &tree.devices {
            for y in tree.devices.iter().filter(|y| y.bus != x.bus) {
                let (gx, gy) = (group_of(&g, x.bus), group_of(&g, y.bus));
                if tree.reaches(x, y) || tree.reaches(y, x) {
                    reached += 1;
                    assert!(gx.is_some() && gx == gy, "seed {seed}: {} and {}", x.bus, y.bus);
                } else if gx != gy {
                    kept_apart += 1;
                }
            }
        }
    }
    assert!(reached > 1000 && kept_apart > 1000, "{reached} reached, {kept_apart} apart");
}

// With every port closed nothing reaches anything and nothing is grouped.
#[test]
fn a_tree_of_closed_ports_has_no_group() {
    for seed in 1..=500u64 {
        let mut tree = Tree::build(&mut drawn(seed.wrapping_mul(0xD1B5_4A32_D192_ED03)));
        tree.ports.iter_mut().for_each(|p| p.closed = true);
        assert!(groups(&tree.ports).is_empty(), "seed {seed}");
        let all = &tree.devices;
        assert!(all.iter().all(|x| all.iter().all(|y| !tree.reaches(x, y))));
    }
}
