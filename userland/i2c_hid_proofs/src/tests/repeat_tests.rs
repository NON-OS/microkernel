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

//! Polled reads serve the last report again whether or not it is new. A
//! verbatim repeat is taken every fourth read, the pad's own report rate,
//! and a run of more than twelve is a stale buffer, taken no more. A read
//! with no pointer report in it, from a collection the driver does not
//! read, ends the run, is posted as nothing and draws no command.

use nonos_i2cmodel::touchpad::Touch;
use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::{take_events, INPUT_KIND_POINTER_REL};

use super::descs::MOUSE;
use super::driven::{bound, frame};
use super::fixture::{of_kind, rig_with, HID_DESC_REG, PAD};
use crate::input::poll;
use crate::setup;

#[test]
fn a_mouse_report_served_thirty_times_is_taken_four_times() {
    let device = HidOverI2c::new(PAD, MOUSE.to_vec(), 2 + 4, 0x04F3, 0x0005);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let mut s = setup::run().expect("setup");
    assert!(s.mouse_layout.is_relative_mouse(), "the mouse was not bound");
    take_events();
    let mut taken = Vec::new();
    for read in 0..30 {
        r.pad.lock().push_input(&[3, 0, 7, 0]);
        poll(&mut s);
        if !of_kind(&take_events(), INPUT_KIND_POINTER_REL).is_empty() {
            taken.push(read);
        }
    }
    assert_eq!(taken, [0, 4, 8, 12]);
}

#[test]
fn a_vendor_report_on_a_pad_with_no_mouse_collection_is_no_pointer_input() {
    let (r, mut s) = bound();
    frame(&r, &mut s, Touch::finger(300, 300));
    take_events();
    r.pad.lock().take_commands();
    r.pad.lock().push_input(&[0x0D, 1, 2, 3]);
    assert!(poll(&mut s), "the report was not read");
    assert!(take_events().is_empty(), "the vendor report was posted");
    assert!(s.repeat.last.is_none(), "the run of the touch before it went on");
    let cmds = r.pad.lock().take_commands();
    let feature = |c: &Command| matches!(c, Command::GetReport { .. } | Command::SetReport { .. });
    assert!(!cmds.iter().any(feature), "the pad was read back for it: {cmds:?}");
}
