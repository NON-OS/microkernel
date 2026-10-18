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

//! `until` at the edges of the clock. Uptime counts whole milliseconds, so a
//! deadline taken late in one can fall due a millisecond short; begun there,
//! the bound still holds (and swfw_tests begins its wait there too). And the
//! look after the deadline sees a part that answered during the last sleep.

use std::cell::Cell;
use std::time::Duration;

use nonos_libc::on_clock;

use crate::constants::timeouts::AUTO_READ_MS;
use crate::init::wait::until;

use super::clock_edge::late_in_a_millisecond;
use super::model::timed;

#[test]
fn a_wait_begun_late_in_a_millisecond_still_waits_the_whole_bound() {
    late_in_a_millisecond();
    let (ok, took) = timed(|| until(AUTO_READ_MS, || false));
    assert!(!ok);
    assert!(took >= Duration::from_millis(AUTO_READ_MS), "gave up after {took:?}");
}

#[test]
fn the_last_look_after_the_deadline_still_counts() {
    // The driver's first look at the deadline comes only once it has passed,
    // so the part's second answer is asked for after the deadline.
    let looks = Cell::new(0u32);
    let _clock = on_clock(move || {
        looks.set(looks.get() + 1);
        if looks.get() == 2 {
            std::thread::sleep(Duration::from_millis(2));
        }
    });
    let asked = Cell::new(0u32);
    let ok = until(0, || {
        asked.set(asked.get() + 1);
        asked.get() == 2
    });
    assert!(ok, "a part that answered during the last sleep is not called silent");
}
