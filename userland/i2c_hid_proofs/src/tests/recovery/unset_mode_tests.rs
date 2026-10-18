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

//! A pad the driver did not set to touchpad mode, with no Input Mode to set
//! or one in a report too long to write, is driven as the mouse it reports
//! as: its mouse reports are posted and never read back as a reset.

use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::{take_events, INPUT_KIND_POINTER_REL};

use crate::input::poll;
use crate::setup;
use crate::tests::bounds_tests::XY;
use crate::tests::descs::{touch_pad, MOUSE, TIP};
use crate::tests::fixture::{of_kind, rig_with, HID_DESC_REG, MAX_INPUT, PAD};

/// Feature report 4: a Surface Switch bit, and no Input Mode.
const SWITCH_ONLY: &[u8] = &[
    0x85, 0x04, 0x05, 0x0D, 0x09, 0x57, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x01, 0xB1, 0x02,
    0x95, 0x07, 0xB1, 0x03,
];

/// Feature report 4: Input Mode after seventy bytes of padding, past the
/// longest feature report the driver builds and writes.
const FAR_MODE: &[u8] = &[
    0x85, 0x04, 0x05, 0x0D, 0x75, 0x08, 0x95, 0x46, 0xB1, 0x03, 0x09, 0x52, 0x15, 0x00, 0x25, 0x0A,
    0x95, 0x01, 0xB1, 0x02,
];

/// A pad with a mouse collection and `feature` in its touch pad, bound,
/// then one mouse report: the commands that report drew.
fn mouse_report_with(feature: &[u8]) -> Vec<Command> {
    let desc = [MOUSE, &touch_pad(&[TIP, XY, feature])].concat();
    let device = HidOverI2c::new(PAD, desc, MAX_INPUT, 0x04F3, 0x3028);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let mut s = setup::run().expect("setup");
    assert!(s.touch_layout.is_absolute_touch(), "the pad was not bound as a touchpad");
    take_events();
    r.pad.lock().take_commands();
    r.pad.lock().push_input(&[3, 0, 5, 0]);
    poll(&mut s);
    assert_eq!(of_kind(&take_events(), INPUT_KIND_POINTER_REL).len(), 1, "the mouse did not move");
    let cmds = r.pad.lock().take_commands();
    cmds
}

fn feature_traffic(cmds: &[Command]) -> usize {
    let feature = |c: &&Command| matches!(c, Command::GetReport { .. } | Command::SetReport { .. });
    cmds.iter().filter(feature).count()
}

#[test]
fn a_pad_with_no_input_mode_is_not_read_back_for_its_mouse_reports() {
    let cmds = mouse_report_with(SWITCH_ONLY);
    assert_eq!(feature_traffic(&cmds), 0, "{cmds:?}");
}

#[test]
fn a_pad_whose_input_mode_could_not_be_written_is_not_read_back_either() {
    let cmds = mouse_report_with(FAR_MODE);
    assert_eq!(feature_traffic(&cmds), 0, "{cmds:?}");
}
