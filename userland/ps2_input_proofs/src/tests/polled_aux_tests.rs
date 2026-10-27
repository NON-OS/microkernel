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

//! A line the broker will not bind leaves its port polled, the mouse's as
//! the keyboard's: the port still comes up, and the serve loop is told to
//! come back for its bytes on the next pass.

use nonos_libc::{acked, present, refuse_lines, released, take_events, INPUT_KIND_POINTER_REL};

use super::controller::{Keyboard, MouseKind};
use super::shared::{machine, AUX, KBD};
use crate::server::context::Context;
use crate::server::pump::pump;
use crate::setup::run;

#[test]
fn a_mouse_whose_line_will_not_bind_is_brought_up_polled() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: false });
    present(&[KBD, AUX]);
    refuse_lines(&[12]);
    let driver = run().expect("ready");
    assert!(driver.mouse_enabled, "the mouse is up without IRQ12");
    assert_eq!((driver.irq_grant_id, driver.aux_irq_grant_id), (101, 0));
    assert!(!released().contains(&AUX.device_id), "the aux claim is kept");
    assert_eq!(acked(), [101], "no line to open for the mouse");
    assert!(driver.polled(), "the loop must not sleep 100 ms between drains");
    let mut ctx = Context::new(driver);
    take_events();
    // A packet: buttons none, sync bit, then dx 5, dy 3.
    ctl.borrow_mut().output.extend([(0x08, true), (5, true), (3, true)]);
    pump(&mut ctx, &mut 0);
    let moved = take_events().iter().any(|e| e.kind == INPUT_KIND_POINTER_REL);
    assert!(moved, "a drain with no interrupt still moves the pointer");
}

#[test]
fn only_a_port_without_a_line_is_polled() {
    let (_ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: false });
    present(&[KBD, AUX]);
    assert!(!run().expect("ready").polled(), "both lines bound: the loop sleeps on them");
    let (_ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: false });
    present(&[KBD, AUX]);
    refuse_lines(&[1]);
    let keyboard = run().expect("ready");
    assert!(keyboard.irq_grant_id == 0 && keyboard.polled(), "the keyboard polled");
}
