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

//! The lock LEDs: the 0xED exchange with a keyboard that answers, one that
//! does not, and one that asks again, while keystrokes keep arriving in the
//! same output buffer.

use nonos_libc::{mk_pio_read, mk_uptime_ms};

use super::controller::{Keyboard, MouseKind};
use super::shared::machine;
use crate::constants::{DATA_OFFSET, KBD_SET_LEDS, STATUS_OFFSET, STATUS_OUTPUT_FULL};
use crate::init::set_leds;
use crate::keymap::locks::{LED_CAPS, LED_NUM};
use crate::prefix::{Prefix, Scan};

/// The configuration byte once the keyboard bring-up turned its port on.
const KEYBOARD_ON: u8 = 0x00;

/// The server's drain, cut down: every byte read, keys kept, a reply given.
fn drain(keys: &mut Vec<u8>) -> Option<u8> {
    let (mut status, mut data, mut p) = (0u32, 0u32, Prefix::IDLE);
    while mk_pio_read(7, STATUS_OFFSET, 1, &mut status) == 0
        && status as u8 & STATUS_OUTPUT_FULL != 0
    {
        let _ = mk_pio_read(7, DATA_OFFSET, 1, &mut data);
        match p.take(data as u8) {
            Scan::Reply(r) => return Some(r),
            _ => keys.push(data as u8),
        }
    }
    None
}

#[test]
fn the_leds_are_set_with_0xed_and_the_bitmap_while_typing_goes_on() {
    let (ctl, _port) = machine(Keyboard::SLOW, MouseKind::Absent);
    ctl.borrow_mut().config = KEYBOARD_ON;
    ctl.borrow_mut().output.push_back((0x1E, false));
    let mut keys = Vec::new();
    assert!(set_leds(7, LED_CAPS | 0xF8, || drain(&mut keys)));
    assert_eq!(ctl.borrow().data_writes(), [KBD_SET_LEDS, LED_CAPS], "only three bits sent");
    assert_eq!(keys, [0x1E], "the keystroke typed meanwhile was delivered");
}

#[test]
fn a_keyboard_that_never_answers_is_given_up_on_in_bounded_time() {
    let (ctl, _port) = machine(Keyboard::DEAD, MouseKind::Absent);
    ctl.borrow_mut().config = KEYBOARD_ON;
    let t = mk_uptime_ms();
    assert!(!set_leds(7, LED_NUM, || drain(&mut Vec::new())));
    assert!(mk_uptime_ms() - t <= 210, "waited {} ms", mk_uptime_ms() - t);
    assert_eq!(ctl.borrow().data_writes(), [KBD_SET_LEDS], "no bitmap without an ACK");
}

#[test]
fn a_resend_is_a_refusal_not_an_ack() {
    let (ctl, _port) = machine(Keyboard::DEAD, MouseKind::Absent);
    ctl.borrow_mut().config = KEYBOARD_ON;
    ctl.borrow_mut().output.push_back((0xFE, false));
    assert!(!set_leds(7, LED_CAPS, || drain(&mut Vec::new())));
}
