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

//! The keyboard side: the port enabled, scanning started, the answer eaten.

use super::controller::{Controller, Mouse};
use super::controller::{Keyboard, MouseKind, CONFIG_KBD_DISABLE};
use super::shared::{attached, machine, FIRMWARE_CONFIG};
use crate::constants::{
    CONFIG_IRQ1, CONFIG_XLATE, CTL_DISABLE_KBD, CTL_ENABLE_KBD, CTL_READ_CONFIG, CTL_WRITE_CONFIG,
    KBD_ENABLE_SCANNING, KBD_RESET,
};
use crate::init::{enable_keyboard, flush_output, keyboard_config};

/// What firmware's 0x30 becomes: interrupt on, clock on, translation on.
const CONFIGURED: u8 = (FIRMWARE_CONFIG | CONFIG_IRQ1 | CONFIG_XLATE) & !CONFIG_KBD_DISABLE;

#[test]
fn the_first_port_is_enabled_then_scanning_started_and_the_ack_consumed() {
    let (ctl, _port) = machine(Keyboard::SLOW, MouseKind::Absent);
    enable_keyboard(7).expect("keyboard up");
    let c = ctl.borrow();
    assert_eq!(
        c.commands(),
        [CTL_DISABLE_KBD, CTL_READ_CONFIG, CTL_WRITE_CONFIG, CTL_ENABLE_KBD],
        "port quiet while the configuration is read, then the port firmware left off turned on"
    );
    assert_eq!(c.data_writes(), [CONFIGURED, KBD_ENABLE_SCANNING]);
    assert_eq!(c.config, CONFIGURED);
    assert_eq!(c.config & CONFIG_KBD_DISABLE, 0);
    assert!(c.output.is_empty(), "the late acknowledgement was waited for and eaten");
}

#[test]
fn a_keyboard_that_never_answers_is_tolerated() {
    let (ctl, _port) = machine(Keyboard::DEAD, MouseKind::Absent);
    enable_keyboard(7).expect("a dead keyboard is not a failed bring-up");
    // Scanning, then the reset that recovers a wedged keyboard, then nothing
    // more once the reset is not acknowledged either.
    assert_eq!(ctl.borrow().data_writes(), [CONFIGURED, KBD_ENABLE_SCANNING, KBD_RESET]);
}

#[test]
fn a_controller_whose_input_buffer_never_drains_is_given_up_on() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    ctl.borrow_mut().input_stuck = true;
    assert_eq!(enable_keyboard(7).err(), Some("kbd input buffer busy"));
    assert!(ctl.borrow().writes.is_empty(), "nothing was written into a full buffer");
}

#[test]
fn without_a_port_behind_the_grant_the_first_read_is_the_error() {
    assert_eq!(enable_keyboard(7).err(), Some("kbd status read failed"));
}

#[test]
fn flushing_drains_stale_bytes_up_to_its_bound() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    for b in 0..5u8 {
        ctl.borrow_mut().output.push_back((b, false));
    }
    flush_output(7);
    assert!(ctl.borrow().output.is_empty());
    for b in 0..40u8 {
        ctl.borrow_mut().output.push_back((b, false));
    }
    flush_output(7);
    assert_eq!(ctl.borrow().output.len(), 24, "sixteen bytes a flush, then stop");
}

#[test]
fn translation_is_turned_on_when_firmware_left_it_off() {
    /*
     * The keymap decodes scan code set 1. A keyboard speaks set 2, and only
     * the controller's translation turns it into set 1: without bit 6 every
     * key decodes as another.
     */
    let (ctl, _port) =
        attached(Controller::new(0x00, Keyboard::PROMPT, Mouse::new(MouseKind::Absent)));
    enable_keyboard(7).expect("keyboard up");
    assert_eq!(ctl.borrow().config, CONFIG_IRQ1 | CONFIG_XLATE);
    assert_eq!(keyboard_config(0xFF), !CONFIG_KBD_DISABLE);
}

#[test]
fn a_configuration_already_right_comes_back_unchanged() {
    // The bring-up's own 0xAD sets bit 4, so the byte it reads always needs
    // that bit cleared again; what is written back is the firmware's byte.
    let (ctl, _port) =
        attached(Controller::new(CONFIGURED, Keyboard::PROMPT, Mouse::new(MouseKind::Absent)));
    enable_keyboard(7).expect("keyboard up");
    assert_eq!(ctl.borrow().config, CONFIGURED);
}

#[test]
fn a_wedged_keyboard_is_reset_and_its_slow_self_test_is_waited_for() {
    /*
     * The self test takes 400 status reads here, past what the spin counts
     * this replaced allowed. The wait is bounded in time instead, and the
     * keyboard comes back scanning.
     */
    let (ctl, _port) = machine(Keyboard::WEDGED, MouseKind::Absent);
    enable_keyboard(7).expect("keyboard up");
    let c = ctl.borrow();
    assert_eq!(c.data_writes(), [CONFIGURED, KBD_ENABLE_SCANNING, KBD_RESET, KBD_ENABLE_SCANNING]);
    assert!(c.output.is_empty(), "the reset ACK, self-test byte and scan ACK were all eaten");
}
