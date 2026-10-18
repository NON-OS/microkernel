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

//! Only an Input Mode read back as other than touchpad mode says a pad
//! reset. A switch the pad turned off is set again, and a mode the pad will
//! not read back is written again, but neither lets go of what it holds,
//! nor counts a try.

use nonos_i2cmodel::touchpad::MODE_REPORT_ID;
use nonos_i2cmodel::Command;
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_UP};

use crate::input::poll;
use crate::state::State;
use crate::tests::fixture::{of_kind, Rig};
use crate::tests::stick_tests::dragging;

/// A mouse report while the pad's mode report reads `feature`; asserts the
/// held click was kept, and gives the SET_REPORTs the report drew.
fn stray_report_with(r: &Rig, s: &mut State, feature: &[u8]) -> Vec<Command> {
    r.pad.lock().set_feature(MODE_REPORT_ID, feature);
    r.pad.lock().push_input(&[3, 0, 5, 0]);
    poll(s);
    let events = take_events();
    let up = of_kind(&events, INPUT_KIND_BUTTON_UP);
    assert!(up.is_empty(), "the held click was let go: {events:?}");
    let sets = r.pad.lock().take_commands().into_iter();
    sets.filter(|c| matches!(c, Command::SetReport { id: MODE_REPORT_ID, .. })).collect()
}

#[test]
fn a_switch_the_pad_turned_off_is_set_again_and_the_drag_kept() {
    let (r, mut s, _) = dragging();
    let sets = stray_report_with(&r, &mut s, &[MODE_REPORT_ID, 3, 0b00]);
    assert_eq!(sets.len(), 1, "the switches were not set again: {sets:?}");
    assert_eq!(r.pad.lock().feature(MODE_REPORT_ID), Some(&[MODE_REPORT_ID, 3, 0b11][..]));
}

#[test]
fn a_mode_the_pad_will_not_read_back_is_written_again_and_the_drag_kept() {
    let (r, mut s, _) = dragging();
    let sets = stray_report_with(&r, &mut s, &[MODE_REPORT_ID]);
    assert_eq!(sets.len(), 1, "the mode was not written again: {sets:?}");
    assert_eq!(r.pad.lock().feature(MODE_REPORT_ID), Some(&[MODE_REPORT_ID, 3, 0b11][..]));
}

#[test]
fn stick_reports_count_no_try_and_a_reset_after_four_is_set_up() {
    let (r, mut s, _) = dragging();
    for dx in 1..=5 {
        mk_idle_ms(1_100);
        if dx == 5 {
            r.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID, 0, 0]);
        }
        r.pad.lock().push_input(&[3, 0, dx, 0]);
        poll(&mut s);
    }
    let mode = r.pad.lock().feature(MODE_REPORT_ID).map(<[u8]>::to_vec);
    assert_eq!(mode, Some(vec![MODE_REPORT_ID, 3, 0b11]), "not set up after four sticks");
}
