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

//! Passive cooling's rule: the step as the formula gives it, the limit held
//! to 0..=100 whatever comes, never falling below _PSV and always climbing
//! back to full there.

use crate::thermal::{step, Constants, FULL, MAX_TC};

const C: Constants = Constants { tc1: 2, tc2: 5, period_ms: 1000 };

#[test]
fn above_psv_the_limit_falls_by_the_formula() {
    // 2 K over _PSV and 2 K up since the last reading: 2×2 + 5×2 = 14 points.
    assert_eq!(step(FULL, C, 80_000, 82_000, 80_000), 86);
    // At _PSV, steady: nothing moves.
    assert_eq!(step(60, C, 80_000, 80_000, 80_000), 60);
    // Over, but cooling fast: the limit may climb back.
    assert_eq!(step(60, C, 90_000, 81_000, 80_000), 60 + 18 - 5);
}

#[test]
fn the_limit_never_leaves_its_range() {
    let wild = [i32::MIN, -273_150, 0, 79_999, 80_000, 80_001, 200_000, i32::MAX];
    let worst = Constants { tc1: MAX_TC as i64, tc2: MAX_TC as i64, period_ms: 100 };
    for limit in [0, 1, 50, 99, 100, 101, 255] {
        for (&last, &now) in wild.iter().flat_map(|a| wild.iter().map(move |b| (a, b))) {
            let next = step(limit, worst, last, now, 80_000);
            assert!(next <= FULL, "{limit} {last} {now} gave {next}");
            if now < 80_000 {
                assert!(next >= limit.min(FULL), "fell below _PSV: {limit} {last} {now}");
            }
        }
    }
}

#[test]
fn below_psv_it_climbs_back_to_full() {
    let flat = Constants { tc1: 1, tc2: 0, period_ms: 1000 };
    let mut limit = 0;
    for n in 0..=FULL {
        if limit == FULL {
            assert!(n > 0);
            return;
        }
        limit = step(limit, flat, 79_000, 79_000, 80_000);
    }
    panic!("still at {limit} after {FULL} steps below _PSV");
}

#[path = "passive_zone_tests.rs"]
mod zone;

#[path = "passive_constants_tests.rs"]
mod constants;
