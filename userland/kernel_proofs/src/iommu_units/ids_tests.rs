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

use super::limits::{domain_count, usable_domain_ids};
use super::shared::shared_domain_count;

const SLOTS: usize = 256;

#[test]
fn nd_zero_leaves_sixteen_ids_however_many_slots_there_are() {
    assert_eq!(usable_domain_ids(domain_count(0), SLOTS), 16);
}

#[test]
fn a_unit_with_more_ids_than_slots_is_capped_at_the_slots() {
    assert_eq!(domain_count(6), 1 << 16);
    assert_eq!(usable_domain_ids(domain_count(6), SLOTS), SLOTS);
    assert_eq!(usable_domain_ids(domain_count(2), SLOTS), SLOTS);
}

#[test]
fn the_unit_with_the_fewest_ids_bounds_every_unit() {
    let shared = shared_domain_count(&[6, 1, 2]);
    assert_eq!(shared, 64);
    assert_eq!(usable_domain_ids(shared, SLOTS), 64);
}

#[test]
fn a_reserved_nd_or_no_unit_leaves_no_id_at_all() {
    assert_eq!(usable_domain_ids(domain_count(7), SLOTS), 0);
    assert_eq!(usable_domain_ids(shared_domain_count(&[]), SLOTS), 0);
    assert_eq!(usable_domain_ids(shared_domain_count(&[6, 7]), SLOTS), 0);
}
