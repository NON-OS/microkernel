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

//! A bound set while the clock cannot be read counts from when its request
//! was taken, which came before the reply: it still refuses what the kernel
//! gave up on, and nothing walks up from uptime 0, which refused every
//! request waiting however long the kernel still waited for it.

use super::model::{io_queue, post};
use super::{sq_tail_doorbell, IoQueue};
use crate::completion_tests::entry;
use crate::error::NvmeError;
use crate::fake::{register, registers};
use crate::regs::Regs;

/// At 300 s of uptime, a read whose command does not complete in time, its
/// next bound set as the clock fails, then `ms` of recovery, and the
/// command's completion seen at last. The queue, and when the reply went.
fn bound_without_a_clock(ms: u64) -> (Regs, IoQueue, u64) {
    nonos_libc::set_clock(1_000);
    let (regs, mut io) = (registers(), io_queue());
    while nonos_libc::mk_uptime_ms() < 300_000 {}
    assert!(io.retried(0, |io| io.transfer(regs, 0, 1, false)).is_err());
    let reply = nonos_libc::mk_uptime_ms() as u64;
    nonos_libc::fail_clock(true);
    io.bound_next(0);
    nonos_libc::fail_clock(false);
    (0..ms / 1_000).for_each(|_| _ = nonos_libc::mk_uptime_ms());
    post(&io, 0, entry(1, 1, 0, 1));
    (regs, io, reply)
}

#[test]
fn a_bound_set_without_a_clock_costs_only_the_request_it_bounds() {
    // The kernel's read waiting through 5 s of recovery is refused, nothing
    // sent; the one it sends as the refusal reaches it is served.
    let (regs, mut io, _) = bound_without_a_clock(5_000);
    post(&io, 1, entry(2, 1, 0, 1));
    let waiting = io.retried(0, |io| io.transfer(regs, 0, 1, false));
    let sent = register(regs, sq_tail_doorbell(1, 0));
    assert!(matches!(waiting, Err(NvmeError::ControllerTimeout)) && sent == 1, "{waiting:?}");
    let resent = io.retried(0, |io| io.transfer(regs, 0, 1, false));
    assert!(resent.is_ok(), "the resent read: {resent:?}");
}

#[test]
fn a_bound_set_without_a_clock_still_refuses_what_the_kernel_gave_up_on() {
    // 300 s of recovery, the kernel queueing a read each 35 s from the
    // reply: none it stopped waiting for gets a command.
    let (regs, mut io, reply) = bound_without_a_clock(300_000);
    for k in 1..=9u64 {
        let sent = register(regs, sq_tail_doorbell(1, 0));
        let r = io.retried(0, |io| io.transfer(regs, 0, 1, false));
        let gave_up = nonos_libc::mk_uptime_ms() as u64 > reply + k * 35_000;
        let unsent = register(regs, sq_tail_doorbell(1, 0)) == sent;
        assert!(!gave_up || unsent, "read {k}, waited for to {}: {r:?}", reply + k * 35_000);
    }
}
