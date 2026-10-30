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

//! Past eight segment 0 units, or off segment 0, a DRHD is left off, and
//! the IOMMU claims nothing while one is.

use super::dmar_keep::{keep, Unit, MAX_REMAP_UNITS};

#[test]
fn eight_segment_0_units_are_kept_and_the_ninth_is_left_off() {
    for kept in 0..MAX_REMAP_UNITS {
        assert_eq!(keep(0, kept), Unit::Kept, "unit {}", kept + 1);
    }
    assert_eq!(keep(0, MAX_REMAP_UNITS), Unit::LeftOff, "the ninth");
    assert_eq!(keep(0, usize::MAX), Unit::LeftOff);
}

#[test]
fn a_unit_off_segment_0_is_left_off_however_few_are_kept() {
    assert_eq!(keep(1, 0), Unit::LeftOff);
    assert_eq!(keep(u16::MAX, 3), Unit::LeftOff);
}
