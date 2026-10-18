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

//! The reset's waits on the uptime clock. Uptime counts whole milliseconds,
//! so a deadline taken late in one can fall due a millisecond short; begun
//! there, each bound still holds, and the look held to the bound's
//! millisecond (held_look) is not the last.

use std::time::{Duration, Instant};

use super::held_look::never_gives_up_at_the_bound;
use crate::init::{hold_ms, until};

#[test]
fn a_wait_begun_late_in_a_millisecond_still_waits_the_whole_bound() {
    never_gives_up_at_the_bound(10, || {
        let start = Instant::now();
        assert!(!until(10, || false), "a part that never answers");
        let took = start.elapsed();
        assert!(took >= Duration::from_millis(10), "gave up after {took:?}");
    });
}

#[test]
fn a_hold_begun_late_in_a_millisecond_still_holds_the_whole_time() {
    never_gives_up_at_the_bound(10, || {
        let start = Instant::now();
        hold_ms(10);
        let took = start.elapsed();
        assert!(took >= Duration::from_millis(10), "held for {took:?}");
    });
}
