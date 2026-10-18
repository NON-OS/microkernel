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

//! The request wait is a budget by the clock. It counted
//! passes: a line firing every 99 ms held a request for 5000 of them, over
//! eight minutes, and a line firing at once gave up on a healthy request in
//! half a second.

use nonos_libc::{now, on_wait, Wake};

use super::model::{Rig, IRQ};
use crate::constants::REQUEST_MS;
use crate::io::{submit, BlkError};
use crate::queue::{Direction, Fault};

#[test]
fn a_request_never_answered_ends_by_the_clock_on_a_line_that_keeps_firing() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    on_wait(|_| Wake { after_ms: 99, fired: true });
    let r = submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1);
    assert_eq!(r, Err(BlkError::Timeout), "a request nobody answered");
    assert!(now() >= REQUEST_MS && now() <= REQUEST_MS + 99, "given up on at {} ms", now());
    assert_eq!(q.stalled(), Some(Fault::TimedOut));
}

#[test]
fn a_healthy_request_on_a_busy_line_is_not_given_up_on() {
    let rig = Rig::modern();
    let mut q = rig.queue();
    let dev = rig.clone();
    let mut n = 0u64;
    on_wait(move |_| {
        n += 1;
        if n == 6000 {
            dev.answer(513);
        }
        Wake { after_ms: u64::from(n.is_multiple_of(10)), fired: true }
    });
    let r = submit(rig.transport(), &mut q, IRQ, Direction::Read, 0, 1);
    assert_eq!(r, Ok(()), "answered at {} ms, under its budget, and given up on", now());
}
