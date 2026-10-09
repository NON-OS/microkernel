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

use super::agaw::AgawLevels;
use super::shared::{all_support, shared_address_width, shared_domain_count, shared_levels};

const SAGAW_3: u64 = 1 << 9;
const SAGAW_4: u64 = 1 << 10;
const SAGAW_5: u64 = 1 << 11;
const SLLPS_2M: u64 = 1 << 34;
const SLLPS_1G: u64 = 1 << 35;

fn mgaw(bits: u64) -> u64 {
    (bits - 1) << 16
}

/* Shaped like a Tiger Lake laptop: the graphics unit and the unit for every
other device, which differ in super page support and domain count. */
const GFX: u64 = SAGAW_4 | SAGAW_5 | SLLPS_2M | (39 - 1) << 16 | 6;
const REST: u64 = SAGAW_4 | SAGAW_5 | SLLPS_2M | SLLPS_1G | (39 - 1) << 16 | 6;

#[test]
fn one_unit_keeps_its_own_choice() {
    assert_eq!(shared_levels(&[SAGAW_3 | SAGAW_4]), Some(AgawLevels::Four));
    assert_eq!(shared_levels(&[SAGAW_3]), Some(AgawLevels::Three));
}

#[test]
fn a_depth_must_be_walkable_by_every_unit() {
    assert_eq!(shared_levels(&[SAGAW_4 | SAGAW_3, SAGAW_3]), Some(AgawLevels::Three));
    assert_eq!(shared_levels(&[SAGAW_4 | SAGAW_5, SAGAW_5]), Some(AgawLevels::Five));
    assert_eq!(shared_levels(&[SAGAW_4, SAGAW_3]), None);
    assert_eq!(shared_levels(&[]), None);
    assert_eq!(shared_levels(&[GFX, REST]), Some(AgawLevels::Four));
}

#[test]
fn a_leaf_size_is_used_only_when_every_unit_has_it() {
    let both = all_support(&[GFX, REST]);
    assert_ne!(both & SLLPS_2M, 0);
    assert_eq!(both & SLLPS_1G, 0, "the graphics unit has no 1 GiB leaves");
}

#[test]
fn snoop_and_coherence_need_every_unit() {
    const COHERENT: u64 = 1;
    const SNOOP: u64 = 1 << 7;
    assert_eq!(all_support(&[COHERENT | SNOOP, COHERENT]), COHERENT);
    assert_eq!(all_support(&[COHERENT, SNOOP]) & (COHERENT | SNOOP), 0);
}

#[test]
fn widths_and_domain_counts_take_the_smallest() {
    assert_eq!(shared_address_width(&[mgaw(48), mgaw(39)]), 39);
    assert_eq!(shared_domain_count(&[2, 6]), 256);
    assert_eq!(shared_domain_count(&[GFX, REST]), 65536);
    assert_eq!(shared_address_width(&[]), 0);
    assert_eq!(shared_domain_count(&[]), 0);
}
