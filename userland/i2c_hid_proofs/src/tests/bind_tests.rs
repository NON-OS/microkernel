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

//! What the report descriptor says decides what is driven: a relative
//! mouse is, by its own collection; a touch screen is no touch pad, and is
//! driven as a screen, never through the touchpad's layout.

use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::{take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_POINTER_REL};

use super::descs::MOUSE;
use super::fixture::{of_kind, rig_with, HID_DESC_REG, PAD};
use crate::hid::Surface;
use crate::input::poll;
use crate::setup;

#[test]
fn a_relative_mouse_is_driven_by_its_own_collection() {
    let device = HidOverI2c::new(PAD, MOUSE.to_vec(), 2 + 4, 0x04F3, 0x0002);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let mut state = setup::run().expect("setup");
    assert!(state.found() && !state.done, "a mouse was released");
    take_events();
    r.pad.lock().push_input(&[3, 0b01, 5, 0xFB]);
    poll(&mut state);
    let events = take_events();
    assert_eq!(of_kind(&events, INPUT_KIND_BUTTON_DOWN).len(), 1, "{events:?}");
    let motion = of_kind(&events, INPUT_KIND_POINTER_REL);
    assert_eq!(motion.iter().map(|e| (e.delta_x, e.delta_y)).collect::<Vec<_>>(), [(5, -5)]);
}

#[test]
fn a_touch_screen_is_no_touch_pad_and_is_driven_as_a_screen() {
    // Usage (Touch Screen), then the touch pad's own finger report.
    let mut desc = nonos_i2cmodel::touchpad::report_descriptor();
    desc[3] = 0x04;
    let device = HidOverI2c::new(PAD, desc, 2 + 8, 0x04F3, 0x0003);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    let state = setup::run().expect("setup");
    assert!(!state.touch_layout.is_absolute_touch(), "a touch screen was bound as a touchpad");
    assert!(state.found() && !state.done, "a touch screen was released");
    assert!(state.mt.surface == Surface::Screen && state.mt.serves());
    let cmds = r.pad.lock().take_commands();
    assert_ne!(cmds.last(), Some(&Command::SetPower { sleep: true }), "{cmds:?}");
}
