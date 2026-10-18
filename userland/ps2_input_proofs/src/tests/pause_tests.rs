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

//! Waiting on a slow controller or device sleeps once the first few polls
//! have not answered, instead of holding a core for the whole bound.

use nonos_libc::{mk_uptime_ms, slept_ms};

use super::controller::{Keyboard, MouseKind};
use super::shared::machine;
use crate::init::read_port;
use crate::init::wait::wait_input_clear;

#[test]
fn a_wait_on_a_busy_controller_sleeps_between_polls() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    ctl.borrow_mut().input_stuck = true;
    let (s, t) = (slept_ms(), mk_uptime_ms());
    assert!(wait_input_clear(7, 500).is_err());
    let waited = (mk_uptime_ms() - t) as u64;
    // The shim's clock moves a millisecond per read too: a third is the floor.
    assert!(3 * (slept_ms() - s) > waited, "slept {} of {waited} ms", slept_ms() - s);
    assert!((500..520).contains(&waited), "the bound is elapsed time: {waited} ms");
}

#[test]
fn a_reply_that_never_comes_is_waited_for_asleep() {
    let (_ctl, _port) = machine(Keyboard::DEAD, MouseKind::Absent);
    let s = slept_ms();
    assert_eq!(read_port(7, false, 200), Ok(None));
    assert!(slept_ms() - s > 50, "slept {} ms", slept_ms() - s);
}

#[test]
fn a_prompt_answer_is_not_slept_on() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    ctl.borrow_mut().output.push_back((0xFA, false));
    let s = slept_ms();
    assert_eq!(read_port(7, false, 200), Ok(Some(0xFA)));
    assert_eq!(slept_ms(), s);
}
