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

//! Frame callbacks at most 30 a second, and an idle window answered at
//! once (wayland/pace_rule.rs; docs/design/box-gl.md, G.6).

#[path = "../../../../capsule_linux/src/linux/wayland/pace_rule.rs"]
mod pace_rule;
use pace_rule::{due, next, PERIOD_MS};

#[test]
fn no_frame_is_answered_before_its_period() {
    assert_eq!(PERIOD_MS, 33, "30 a second");
    assert_eq!(due(Some(1_000), 1_000), 1_033, "a client drawing at once waits a period");
    assert_eq!(due(Some(1_000), 1_020), 1_033);
    let mut last = 0;
    let mut answered = 0;
    for now in 0..1_000 {
        if due(Some(last), now) <= now {
            (last, answered) = (now, answered + 1);
        }
    }
    assert!(answered <= 31, "{answered} in a second");
}

#[test]
fn an_idle_window_is_answered_at_once() {
    assert_eq!(due(None, 5), 5, "never answered");
    assert_eq!(due(Some(1_000), 1_033), 1_033);
    assert_eq!(due(Some(1_000), 9_000), 9_000, "idle long past a period");
}

#[test]
fn the_loop_wakes_for_the_first_surface_due() {
    assert_eq!(next([].into_iter(), 100), None, "no surface holds any");
    assert_eq!(next([Some(90), Some(80)].into_iter(), 100), Some(13));
    assert_eq!(next([Some(50), Some(90)].into_iter(), 100), Some(0), "one due now");
}
