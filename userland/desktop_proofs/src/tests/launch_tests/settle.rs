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

//! When a launch starts, fails or runs out of time.

use crate::launch::timing::{DEADLINE_MS, FIRST_PROBE_MS};
use crate::launch::{Launch, Step};
use crate::state::Uptime;

pub(super) fn fresh() -> Launch {
    Launch::new(b"notes", vec![3, 4], Uptime(1_000))
}

#[test]
fn a_new_launch_waits_before_its_first_probe() {
    let l = fresh();
    assert_eq!(l.step(Uptime(1_000), None), Step::Wait);
    assert_eq!(l.step(Uptime(1_000 + FIRST_PROBE_MS - 1), None), Step::Wait);
    assert_eq!(l.step(Uptime(1_000 + FIRST_PROBE_MS), None), Step::Probe);
}

#[test]
fn the_app_appearing_starts_it_at_any_point() {
    let mut l = fresh();
    assert_eq!(l.step(Uptime(1_010), Some(42)), Step::Started(42));
    l.probed(Uptime(1_600), Some(0));
    assert_eq!(l.step(Uptime(1_600), Some(42)), Step::Started(42));
    assert_eq!(l.step(Uptime(1_000 + DEADLINE_MS + 5), Some(42)), Step::Started(42));
}

#[test]
fn an_answered_probe_with_no_app_fails_it() {
    let mut l = fresh();
    l.probed(Uptime(1_500), Some(0));
    assert!(l.served());
    assert_eq!(l.step(Uptime(1_500), None), Step::Failed);
}

#[test]
fn the_deadline_fails_a_launch_that_never_settles() {
    let l = fresh();
    assert_ne!(l.step(Uptime(1_000 + DEADLINE_MS - 1), None), Step::Failed);
    assert_eq!(l.step(Uptime(1_000 + DEADLINE_MS), None), Step::Failed);
}
