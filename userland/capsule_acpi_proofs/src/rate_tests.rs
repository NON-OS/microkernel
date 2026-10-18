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

//! One controller's event rate: `PER_SECOND` events admitted in a second,
//! the next refused and counted, and the count starting afresh once a
//! second has passed or the clock reads earlier than the window.

use crate::rate::{Rate, PER_SECOND};

#[test]
fn a_second_holds_per_second_events() {
    let mut r = Rate::default();
    for i in 0..PER_SECOND {
        assert!(r.admit(5_000 + u64::from(i)), "event {i}");
    }
    assert!(!r.admit(5_500));
    assert!(!r.admit(5_999));
    assert_eq!(r.refused, 2);
    assert!(r.admit(6_000), "a new second");
    assert_eq!(r.count, 1);
}

#[test]
fn a_clock_that_goes_back_starts_afresh() {
    let mut r = Rate::default();
    (0..PER_SECOND).for_each(|_| assert!(r.admit(9_000)));
    assert!(!r.admit(9_000));
    assert!(r.admit(8_000));
    assert_eq!((r.window_ms, r.count, r.refused), (8_000, 1, 1));
}
