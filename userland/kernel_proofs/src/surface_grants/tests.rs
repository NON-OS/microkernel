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

use super::covering::covering;

const PAGE: u64 = 4096;
const VA: u64 = 0x7000_0000_0000;
const RUN: u64 = 0x1_2000_0000;

/// One scanout grant of eight pages, and the next grant right after it.
fn grants() -> [(u64, u64, u64); 2] {
    [(VA, 8 * PAGE, RUN), (VA + 8 * PAGE, 4 * PAGE, RUN + 0x100_0000)]
}

#[test]
fn a_scanout_inside_its_grant_names_the_grants_run() {
    assert_eq!(covering(grants(), VA, 8 * PAGE), Some((RUN, 8 * PAGE)));
    assert_eq!(covering(grants(), VA + PAGE, 2 * PAGE), Some((RUN, 8 * PAGE)));
    assert_eq!(covering(grants(), VA + 8 * PAGE, PAGE), Some((RUN + 0x100_0000, 4 * PAGE)));
}

#[test]
fn a_range_across_two_grants_is_neither() {
    // Adjacent windows still come back to the broker as two runs.
    assert_eq!(covering(grants(), VA + 7 * PAGE, 2 * PAGE), None);
    assert_eq!(covering(grants(), VA, 12 * PAGE), None);
}

#[test]
fn a_range_outside_every_grant_is_refused() {
    assert_eq!(covering(grants(), VA - PAGE, 2 * PAGE), None);
    assert_eq!(covering(grants(), VA + 12 * PAGE, PAGE), None);
    assert_eq!(covering([], VA, PAGE), None, "a caller with no grant");
}

#[test]
fn empty_and_wrapping_ranges_are_refused() {
    assert_eq!(covering(grants(), VA, 0), None);
    assert_eq!(covering(grants(), u64::MAX - PAGE, 2 * PAGE), None);
    // A grant record whose window would wrap matches nothing.
    assert_eq!(covering([(u64::MAX - PAGE, 2 * PAGE, RUN)], u64::MAX - PAGE, PAGE), None);
}
