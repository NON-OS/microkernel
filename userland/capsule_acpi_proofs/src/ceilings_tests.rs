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

//! F-100: the ceilings handed to the kernel. A processor several zones
//! slow takes the lowest; one no zone limits any more goes back to 100;
//! only changes are sent; and what the kernel holds drops a lifted limit.

#[path = "../../capsule_acpi/src/cooling/ceilings.rs"]
mod rule;

use rule::{changes, record, FULL};

#[test]
fn the_lowest_limit_wins_and_only_changes_are_sent() {
    let (a, b) = ([0u64, 1], [1u64, 2]);
    let zones = [(&a[..], 70), (&b[..], 40)];
    assert_eq!(changes(&zones, &[]), [(0, 70), (1, 40), (2, 40)]);
    assert_eq!(changes(&zones, &[(0, 70), (1, 40), (2, 40)]), []);
    assert_eq!(changes(&zones, &[(0, 70), (1, 50), (2, 40)]), [(1, 40)]);
    let other_way = [(&b[..], 40), (&a[..], 70)];
    assert_eq!(changes(&other_way, &[]), [(0, 70), (1, 40), (2, 40)], "in either order");
}

#[test]
fn a_processor_no_zone_limits_goes_back_to_full() {
    let a = [0u64];
    assert_eq!(changes(&[(&a[..], 100)], &[(0, 60), (5, 30)]), [(0, FULL), (5, FULL)]);
    assert_eq!(changes(&[], &[]), []);
    assert_eq!(changes(&[(&a[..], 100)], &[]), [], "full is the kernel's default");
}

#[test]
fn a_limit_past_full_is_sent_as_full() {
    let a = [3u64];
    assert_eq!(changes(&[(&a[..], 250)], &[(3, 10)]), [(3, FULL)]);
}

#[test]
fn the_record_matches_what_the_kernel_holds() {
    let mut sent = Vec::new();
    record(&mut sent, 1, 40);
    record(&mut sent, 1, 30);
    record(&mut sent, 2, 90);
    assert_eq!(sent, [(1, 30), (2, 90)]);
    record(&mut sent, 1, FULL);
    assert_eq!(sent, [(2, 90)], "a lifted limit leaves no entry, as in the kernel");
}
