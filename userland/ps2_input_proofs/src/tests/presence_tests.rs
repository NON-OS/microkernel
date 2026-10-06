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

//! Whether a controller is there at all, and what a failed bring-up gives
//! back. The broker lists the keyboard record on every machine, so the
//! record is not presence; the controller answering is.

use nonos_libc::{attach, present, released, Port};

use super::controller::{Keyboard, MouseKind};
use super::shared::{machine, AUX, KBD};
use crate::discover::find_ps2_kbd;
use crate::init::flush_output;
use crate::setup::{controller_answers, run};

/// The ports of a machine with no i8042: nothing drives the bus, every read
/// is all ones, and writes go nowhere.
struct Floating;

impl Port for Floating {
    fn read(&mut self, _offset: u16) -> Option<u8> {
        Some(0xFF)
    }

    fn write(&mut self, _offset: u16, _value: u8) -> bool {
        true
    }
}

fn released_since(before: usize) -> Vec<u64> {
    released()[before..].to_vec()
}

#[test]
fn a_floating_bus_never_empties_its_output_buffer() {
    let _port = attach(Box::new(Floating));
    assert!(!flush_output(7));
}

#[test]
fn a_controller_with_a_few_stale_bytes_empties_and_answers() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    for b in 0..3u8 {
        ctl.borrow_mut().output.push_back((b, false));
    }
    assert!(flush_output(7));
    assert!(ctl.borrow().output.is_empty());
}

#[test]
fn a_machine_without_an_i8042_is_absent_and_the_probe_gives_its_claim_back() {
    let _port = attach(Box::new(Floating));
    present(&[KBD, AUX]);
    let dev = find_ps2_kbd().expect("the broker lists the record anyway");
    let before = released().len();
    assert!(!controller_answers(dev));
    assert_eq!(released_since(before), [KBD.device_id]);
}

#[test]
fn a_machine_with_a_controller_is_present_and_the_probe_gives_its_claim_back() {
    let (_ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: false });
    present(&[KBD, AUX]);
    let dev = find_ps2_kbd().expect("listed");
    let before = released().len();
    assert!(controller_answers(dev));
    assert_eq!(released_since(before), [KBD.device_id], "the probe holds nothing afterwards");
}

#[test]
fn a_keyboard_that_cannot_be_enabled_gives_back_both_claims() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: false });
    ctl.borrow_mut().input_stuck = true;
    present(&[KBD, AUX]);
    let before = released().len();
    assert_eq!(run().err(), Some("kbd input buffer busy"));
    assert_eq!(
        released_since(before),
        [AUX.device_id, KBD.device_id],
        "the aux claim and its line, then the keyboard claim with its port grant and line"
    );
}

#[test]
fn a_keyboard_alone_that_cannot_be_enabled_gives_back_its_claim() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    ctl.borrow_mut().input_stuck = true;
    present(&[KBD]);
    let before = released().len();
    assert!(run().is_err());
    assert_eq!(released_since(before), [KBD.device_id]);
}

#[test]
fn a_working_bring_up_keeps_its_claims() {
    let (_ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: true });
    present(&[KBD, AUX]);
    let before = released().len();
    run().expect("ready");
    assert!(released_since(before).is_empty());
}
