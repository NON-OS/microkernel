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

//! Only the Input Mode's own report decides whether the mode read back. A
//! pad whose Surface Switch sits in a report of its own that never reads
//! back, its mode reading back, is a pad that reads back: a still finger's
//! click outlasts any run of stick reports there.

use nonos_i2cmodel::touchpad::{report_descriptor, touch_report, Touch};
use nonos_i2cmodel::HidOverI2c;
use nonos_libc::{mk_idle_ms, take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use crate::input::poll;
use crate::setup;
use crate::tests::descs::MOUSE;
use crate::tests::fixture::{of_kind, rig_with, HID_DESC_REG, MAX_INPUT, PAD};

/// A device configuration collection with feature report 4: the Surface
/// Switch, a bit, ahead of the touch pad's own, so it is the one taken.
const OWN_SWITCH: &[u8] = &[
    0x05, 0x0D, 0x09, 0x0E, 0xA1, 0x01, 0x85, 0x04, 0x09, 0x57, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01,
    0x95, 0x01, 0xB1, 0x02, 0x95, 0x07, 0xB1, 0x03, 0xC0,
];

#[test]
fn a_switch_report_that_never_reads_back_arms_nothing() {
    let desc = [MOUSE, OWN_SWITCH, &report_descriptor()].concat();
    let r =
        rig_with(HidOverI2c::new(PAD, desc, MAX_INPUT, 0x04F3, 0x3028), Some((PAD, HID_DESC_REG)));
    let mut s = setup::run().expect("setup");
    assert_eq!(s.touch_layout.surface_switch_report_id, 4, "the switch is not in its own report");
    let press = Touch { button: true, ..Touch::finger(300, 300) };
    for x in [300, 301] {
        r.pad.lock().push_input(&touch_report(&Touch { x, ..press }));
        poll(&mut s);
    }
    assert_eq!(of_kind(&take_events(), INPUT_KIND_BUTTON_DOWN).len(), 1, "no click held");
    // The finger holds still and the pad says nothing; a stick moves.
    let mut ups = 0;
    for dx in 1..=70 {
        r.pad.lock().set_feature(4, &[4]);
        r.pad.lock().push_input(&[3, 0, dx, 0]);
        poll(&mut s);
        ups += of_kind(&take_events(), INPUT_KIND_BUTTON_UP).len();
        mk_idle_ms(8);
    }
    assert_eq!(ups, 0, "a switch report that did not read back let go of the click");
}
