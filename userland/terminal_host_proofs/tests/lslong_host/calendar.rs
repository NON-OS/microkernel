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

//! Leap days and month boundaries in the stamp.

use crate::support::at;

/// Leap days are where an era-based conversion goes wrong if the century rule
/// is missed: 2000 and 2024 are leap years, 2100 is not.
#[test]
fn leap_days_land_on_february_twentynine() {
    assert_eq!(at(951_782_400_000), "Feb 29 00:00");
    assert_eq!(at(1_709_164_800_000), "Feb 29 00:00");
    assert_eq!(at(1_709_251_200_000), "Mar  1 00:00");
    assert_eq!(at(4_107_456_000_000), "Feb 28 00:00");
    assert_eq!(at(4_107_542_400_000), "Mar  1 00:00");
}

#[test]
fn month_boundaries_do_not_slip_a_day() {
    for (ms, want) in [
        (1_735_689_599_000u64, "Dec 31 23:59"),
        (1_735_689_600_000, "Jan  1 00:00"),
        (1_738_367_999_000, "Jan 31 23:59"),
        (1_738_368_000_000, "Feb  1 00:00"),
    ] {
        assert_eq!(at(ms), want, "{ms}");
    }
}
