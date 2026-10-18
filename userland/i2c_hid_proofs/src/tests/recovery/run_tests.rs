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

//! The run that lets go on a pad whose mode never reads back is counted from
//! its first mouse report: 100 ms from that one, however long the pad had
//! been silent before it; and its fourth report is enough, not a fifth.

use nonos_i2cmodel::touchpad::MODE_REPORT_ID;
use nonos_libc::{mk_idle_ms, mk_uptime_ms, take_events, INPUT_KIND_BUTTON_UP};

use crate::input::poll;
use crate::state::State;
use crate::tests::fixture::{of_kind, Rig};
use crate::tests::stick_tests::dragging;

/// `n` stick reports `every_ms` apart, the mode never reading back: when the
/// first button-up came, in ms from the first report, if one did.
fn sticks(r: &Rig, s: &mut State, n: u8, every_ms: u64) -> Option<i64> {
    let start = mk_uptime_ms();
    for dx in 1..=n {
        r.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
        r.pad.lock().push_input(&[3, 0, dx, 0]);
        poll(s);
        if !of_kind(&take_events(), INPUT_KIND_BUTTON_UP).is_empty() {
            return Some(mk_uptime_ms() - start);
        }
        mk_idle_ms(every_ms);
    }
    None
}

#[test]
fn the_100_ms_are_counted_from_the_first_mouse_report_not_the_last_touch() {
    let (r, mut s, _) = dragging();
    // The finger holds still and the pad says nothing for half a second.
    mk_idle_ms(500);
    let up = sticks(&r, &mut s, 30, 8);
    assert!(up.is_some_and(|ms| ms >= 100), "let go {up:?} ms into the run");
}

#[test]
fn the_fourth_mouse_report_of_a_slow_run_lets_go() {
    let (r, mut s, _) = dragging();
    assert_eq!(sticks(&r, &mut s, 3, 50), None, "let go before the fourth report");
    let (r, mut s, _) = dragging();
    assert!(sticks(&r, &mut s, 4, 50).is_some(), "not let go at the fourth report");
}
