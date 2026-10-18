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

//! A request given up on. It stayed the device's and every
//! later request spent its own whole budget waiting for the device to
//! catch up, then failed with E_IO, until reboot. Now the queue stalls,
//! naming why: the next request is refused at once, and the server resets
//! the device before it (`server::recover`, held in `order_tests`).

use nonos_libc::{now, waits};

use super::model::{Rig, IRQ};
use crate::constants::REQUEST_MS;
use crate::io::{submit, BlkError};
use crate::queue::{Direction, Fault};

#[test]
fn a_request_given_up_on_costs_the_next_one_nothing() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    let first = submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1);
    assert_eq!(first, Err(BlkError::Timeout));
    assert_eq!(now(), REQUEST_MS);
    let (before, slept) = (now(), waits());
    let next = submit(rig.transport(), &mut q, IRQ, Direction::Flush, 0, 0);
    assert_eq!(next, Err(BlkError::Io), "a request was posted over one still the device's");
    assert_eq!((now() - before, waits() - slept), (0, 0), "the next request waited");
    assert_eq!(q.stalled(), Some(Fault::TimedOut), "the first reason is the one kept");
    assert_eq!(rig.posted(), 1, "nothing was posted over the request in flight");
}

#[test]
fn a_request_answered_late_is_still_not_taken_for_the_next() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    let _ = submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1);
    rig.answer(513);
    let next = submit(rig.transport(), &mut q, IRQ, Direction::Read, 8, 1);
    assert_eq!(next, Err(BlkError::Io), "only a reset ends a stalled queue");
    let mut out = [0u8; 512];
    assert!(!q.take(&mut out), "the late answer's buffer was handed out");
}
