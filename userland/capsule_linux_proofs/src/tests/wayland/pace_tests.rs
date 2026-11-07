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

//! Frame callbacks at most once a display refresh, the one the compositor
//! reports or 60 a second, and an idle window answered at once (wayland/pace_rule.rs; the box GL design,
//! G.6).

#[path = "../../../../capsule_linux/src/linux/wayland/pace_rule.rs"]
mod pace_rule;
use pace_rule::{due, next, period, refresh_mhz, DEFAULT_PERIOD_NS};

const P60: u64 = DEFAULT_PERIOD_NS;

#[test]
fn no_frame_is_answered_before_the_next_refresh() {
    assert_eq!(refresh_mhz(P60), 60_000, "the refresh wl_output says");
    // Slots begin at 1000, 1017, 1034, 1050: a sixtieth, in whole ms.
    assert_eq!(due(Some(1_000), 1_000, P60), 1_017, "a client drawing at once waits a slot");
    assert_eq!(due(Some(1_000), 1_010, P60), 1_017);
    assert_eq!(due(Some(1_017), 1_017, P60), 1_034);
    assert_eq!(due(Some(1_034), 1_034, P60), 1_050);
}

#[test]
fn a_client_drawing_at_once_is_answered_sixty_times_a_second() {
    for start in [0, 7, 1_000, 123_456] {
        let mut last = start;
        let mut answered = 0;
        for now in start + 1..start + 1_001 {
            if due(Some(last), now, P60) <= now {
                (last, answered) = (now, answered + 1);
            }
        }
        assert!((59..=61).contains(&answered), "{answered} in a second from {start}");
    }
}

#[test]
fn an_idle_window_is_answered_at_once() {
    assert_eq!(due(None, 5, P60), 5, "never answered");
    assert_eq!(due(Some(1_000), 1_017, P60), 1_017);
    assert_eq!(due(Some(1_000), 9_000, P60), 9_000, "idle long past a slot");
}

#[test]
fn the_loop_wakes_for_the_first_surface_due() {
    assert_eq!(next([].into_iter(), 100, P60), None, "no surface holds any");
    // 90's next slot is 100, 80's is 84: both due now or past.
    assert_eq!(next([Some(90), Some(80)].into_iter(), 100, P60), Some(0));
    assert_eq!(next([Some(101)].into_iter(), 101, P60), Some(16), "117 is the slot after 101");
}

#[test]
fn a_reported_refresh_paces_at_its_own_rate() {
    let p144 = period(1_000_000_000 / 144);
    assert_eq!(refresh_mhz(p144), 144_000);
    let mut last = 1_000;
    let mut answered = 0;
    for now in 1_001..2_001 {
        if due(Some(last), now, p144) <= now {
            (last, answered) = (now, answered + 1);
        }
    }
    assert!((143..=145).contains(&answered), "{answered} in a second at 144 Hz");
}

#[test]
fn a_report_out_of_reason_is_taken_for_none() {
    assert_eq!(period(0), DEFAULT_PERIOD_NS, "not known");
    assert_eq!(period(1_000), DEFAULT_PERIOD_NS, "a million a second");
    assert_eq!(period(1_000_000_000), DEFAULT_PERIOD_NS, "once a second");
    assert_eq!(period(16_000_000), 16_000_000);
}
