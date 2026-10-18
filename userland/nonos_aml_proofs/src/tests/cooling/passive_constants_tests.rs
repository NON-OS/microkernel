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

//! Passive cooling's constants: absent, past their bounds, a _TSP of 0 or
//! two zero constants leave the zone report-only.

use crate::thermal::{constants, Constants, MAX_TC, MAX_TSP};

#[test]
fn constants_absent_or_lying_leave_the_zone_report_only() {
    assert_eq!(
        constants(Some(2), Some(5), Some(10)),
        Some(Constants { tc1: 2, tc2: 5, period_ms: 1000 })
    );
    assert_eq!(constants(Some(MAX_TC), Some(0), Some(MAX_TSP)).map(|c| c.period_ms), Some(300_000));
    for (tc1, tc2, tsp) in [
        (None, Some(5), Some(10)),
        (Some(2), None, Some(10)),
        (Some(2), Some(5), None),
        (Some(MAX_TC + 1), Some(5), Some(10)),
        (Some(2), Some(u64::MAX), Some(10)),
        (Some(2), Some(5), Some(0)),
        (Some(2), Some(5), Some(MAX_TSP + 1)),
        (Some(0), Some(0), Some(10)),
    ] {
        assert_eq!(constants(tc1, tc2, tsp), None, "{tc1:?} {tc2:?} {tsp:?}");
    }
}
