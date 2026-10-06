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

//! When the installer's disk list looks again on its own
//! (`capsule_install/src/install/rescan.rs`). The list was made once, when
//! the disks screen opened, so a driver that came up a moment later left
//! the screen saying there was no disk until the machine was restarted.
//! It now looks every two seconds while it has nothing to install to, and
//! a look held up by a driver that does not answer is spaced out so the
//! screen still takes keys between looks.

#[path = "../../capsule_install/src/install/rescan.rs"]
mod rescan;

use rescan::{due, EVERY_MS};

#[test]
fn a_quick_look_is_repeated_every_two_seconds() {
    assert_eq!(EVERY_MS, 2000);
    assert!(!due(10_000, 10_000, 5));
    assert!(!due(11_999, 10_000, 5));
    assert!(due(12_000, 10_000, 5));
    assert!(due(60_000, 10_000, 0));
}

#[test]
fn a_slow_look_waits_four_times_what_it_took() {
    /* A driver that held the look for its whole eight second timeout. */
    assert!(!due(10_000 + 31_999, 10_000, 8000));
    assert!(due(10_000 + 32_000, 10_000, 8000));
    /* Under half a second taken, the two seconds stand. */
    assert!(due(12_000, 10_000, 500));
}

#[test]
fn a_clock_that_runs_back_or_wraps_never_looks_in_a_loop_or_panics() {
    assert!(!due(5_000, 10_000, 5));
    assert!(!due(i64::MIN, i64::MAX, 5));
    assert!(due(i64::MAX, i64::MIN, 5), "saturates rather than wrapping");
    assert!(!due(0, 0, i64::MAX));
    assert!(due(2000, 0, -50), "a negative duration counts as none");
}
