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

//! A unit walks the kernel's four-level tables only when EFR.HATS says so,
//! and a reserved HATS walks none.

use super::amd_efr::{hats_levels, walks};

const HATS_SHIFT: u32 = 10;

#[test]
fn hats_names_four_five_or_six_levels() {
    assert_eq!(hats_levels(0), Some(4));
    assert_eq!(hats_levels(1 << HATS_SHIFT), Some(5));
    assert_eq!(hats_levels(2 << HATS_SHIFT), Some(6));
    assert_eq!(hats_levels(3 << HATS_SHIFT), None, "reserved");
    assert_eq!(hats_levels(!(3 << HATS_SHIFT)), Some(4), "other bits are not HATS");
}

#[test]
fn mode_4_needs_a_hats_that_reaches_it() {
    assert!(walks(0, 4));
    assert!(walks(2 << HATS_SHIFT, 4));
    assert!(!walks(3 << HATS_SHIFT, 4), "a reserved HATS is refused");
    assert!(!walks(0, 5), "four levels do not walk a five-level table");
}
