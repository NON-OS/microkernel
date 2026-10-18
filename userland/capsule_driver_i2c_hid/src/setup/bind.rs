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
use super::report_desc::read_report_descriptor;
use super::say::{say, say_retrying};
use crate::drive::configure;
use crate::hid::{input_len, input_register, parse_mouse_layout, parse_report_descriptor};
use crate::hid::{sleep, wake, Multitouch, Surface, HID_DESC_LEN};
use crate::state::State;

/// Bind the device whose HID descriptor was just read at `addr`: wake it,
/// read its report descriptor, and drive it when that declares a touch pad,
/// a touch screen or a relative mouse. Anything else (an I2C keyboard, a
/// sensor hub) is put back to sleep and released: its reports are never
/// read as pointer input, and the driver leaves.
pub(super) fn bind(state: &mut State, addr: u8) {
    state.addr = addr;
    state.input_register = input_register(&state.descriptor);
    state.input_len = input_len(&state.descriptor);
    // Wake the device before touching the report descriptor: a real
    // touchpad answers nothing until SET_POWER(ON) + RESET.
    state.woke = wake(state.i2c_port, addr, &state.descriptor, state.input_register);
    let Some(report_desc) = read_report_descriptor(state) else {
        // Bound only once the report descriptor reads: without it the
        // reports cannot be decoded, and a later reprobe gets another try.
        say_retrying(state, "driver.i2c_hid: the touchpad answered but its report descriptor could not be read; retrying\n");
        return;
    };
    state.touch_layout = parse_report_descriptor(&report_desc);
    state.mouse_layout = parse_mouse_layout(&report_desc);
    state.mt = Multitouch::new(&report_desc);
    let touchpad = state.touch_layout.is_absolute_touch();
    let screen = state.mt.surface == Surface::Screen && state.mt.serves();
    if !touchpad && !screen && !state.mouse_layout.is_relative_mouse() {
        let _ = sleep(state.i2c_port, addr, &state.descriptor);
        say("driver.i2c_hid: the HID-over-I2C device found is no touchpad, touchscreen or mouse (a keyboard or a sensor hub); left asleep, not driven\n");
        state.done = true;
        return;
    }
    state.descriptor_len = HID_DESC_LEN;
    // Input mode = touchpad (the only collection PTP-class pads generate
    // reports on), surface and button switches on, read-modify-write per
    // feature report, so a pad left muted or mode-switched by a previous
    // session is repaired and a correct one sees no writes.
    configure(state);
    let kind = match (touchpad, screen) {
        (_, true) => "a touchscreen",
        (true, false) => "a precision touchpad",
        (false, false) => "a mouse",
    };
    let reset = if state.woke { "reset acknowledged" } else { "no reset report" };
    crate::diag::line(alloc::format!(
        "driver.i2c_hid: bound at {addr:#04x}, read as {kind}; {reset}\n"
    ));
}
