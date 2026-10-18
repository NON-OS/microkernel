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

//! A set-up the bus fails is tried again. One NACKed write leaves reset
//! detection on for the session, and a set-up slower than the holdoff
//! still waits a holdoff from its end before the next.

use nonos_i2cmodel::touchpad::MODE_REPORT_ID;
use nonos_i2cmodel::Command;
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_UP};

use super::behind::{dragging, mouse_reports};
use crate::input::poll;
use crate::tests::fixture::of_kind;

#[test]
fn one_nacked_write_leaves_reset_detection_on() {
    let (f, mut s) = dragging();
    // A stick report while the mode does not read back; its write is NACKed.
    f.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
    f.nacks.set(1);
    mouse_reports(&f, &mut s, 1);
    assert_eq!(f.nacks.get(), 0, "no write was NACKed");
    // Later the pad really resets, its mode read back as the mouse's.
    mk_idle_ms(5_000);
    f.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID, 0, 0]);
    mouse_reports(&f, &mut s, 30);
    let mode = f.pad.lock().feature(MODE_REPORT_ID).map(<[u8]>::to_vec);
    assert_eq!(mode, Some(vec![MODE_REPORT_ID, 3, 0b11]), "the mode was not set again");
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len(), 1, "the click stayed down");
}

#[test]
fn a_set_up_slower_than_the_holdoff_waits_a_holdoff_from_its_end() {
    let (f, mut s) = dragging();
    // Every set-up writes, as the mode never reads back; each write 1.5 s.
    f.slow_ms.set(1_500);
    f.pad.lock().take_commands();
    for dx in 1..=3 {
        f.pad.lock().set_feature(MODE_REPORT_ID, &[MODE_REPORT_ID]);
        f.pad.lock().push_input(&[3, 0, dx, 0]);
        poll(&mut s);
    }
    let set = |c: &&Command| matches!(c, Command::SetReport { .. });
    let sets = f.pad.lock().take_commands().iter().filter(set).count();
    assert_eq!(sets, 1, "set up back to back");
}
