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

//! A HID-over-I2C device that is neither touchpad nor mouse where the pad
//! was looked for: an I2C keyboard. Its descriptor says what it is, so it is
//! left asleep, and no report of it is ever read as pointer input.

use nonos_i2cmodel::{Command, HidOverI2c};
use nonos_libc::take_events;

use super::descs::{CTRL_A, KEYBOARD};
use super::fixture::{rig_with, HID_DESC_REG, PAD};
use crate::input::poll;
use crate::setup;

fn keyboard() -> HidOverI2c {
    HidOverI2c::new(PAD, KEYBOARD.to_vec(), 2 + CTRL_A.len() as u16, 0x04F3, 0x0001)
}

#[test]
fn a_keyboards_ctrl_and_keys_never_click_or_move_the_pointer() {
    let r = rig_with(keyboard(), Some((PAD, HID_DESC_REG)));
    let mut state = setup::run().expect("setup");
    take_events();
    for _ in 0..4 {
        r.pad.lock().push_input(&CTRL_A);
        poll(&mut state);
    }
    let events = take_events();
    assert!(events.is_empty(), "a keyboard report was posted as {events:?}");
}

#[test]
fn a_keyboard_is_released_asleep_and_left_unbound() {
    let r = rig_with(keyboard(), Some((PAD, HID_DESC_REG)));
    let state = setup::run().expect("setup");
    assert!(!state.found(), "a keyboard was bound as the touchpad");
    let cmds = r.pad.lock().take_commands();
    let reset = cmds.iter().position(|c| *c == Command::Reset).expect("woken to be read");
    let sleep = cmds.iter().rposition(|c| *c == Command::SetPower { sleep: true });
    assert!(sleep.is_some_and(|s| s > reset), "the keyboard was left awake: {cmds:?}");
}
