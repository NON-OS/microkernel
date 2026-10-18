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

//! A kernel request queued while another sender's read waited on a command
//! that then stalled. The recovery follows that sender's reply, but the
//! kernel's request was queued during its wait, so the bound counts from
//! when that read was taken (nvm::bound): the kernel's request is answered
//! within the 35 s the kernel waits, or refused with nothing sent. Counted
//! from the other's reply, a kernel write went out after the kernel had
//! reported it lost.

use super::inbox::{Inbox, Reply};
use super::server::Server;
use crate::fake::{register, up};
use crate::nvm::sq_tail_doorbell;
use crate::protocol::E_TIMEDOUT;

/// How long the kernel waits for a reply (SLOW_BUDGET_MS).
const KERNEL_WAIT_MS: u64 = 35_000;
const INSTALLER: u32 = 42;

/// The installer's read stalls for its 30 s and is recovered, RDY `slow`
/// clock reads behind CC.EN; the kernel's request, a write if `writes`,
/// was queued `into` ms after the installer's and takes `hold` reads. Its
/// reply, the ms from its queueing to it, and the commands sent before it.
fn behind_a_stall(slow: u64, into: u64, hold: u64, writes: bool) -> (Reply, u64, u32) {
    let mut s = Server::new(up());
    if slow != 0 {
        s.device.info.cap |= 0xff << 24;
        s.fake.borrow_mut().slow = slow;
    }
    (s.fake.borrow_mut().stall, s.fake.borrow_mut().mute) = (vec![1], true);
    let t0 = s.fake.borrow().ticks * 100;
    let mut inbox = Inbox::from([(t0, INSTALLER), (t0 + into, 0)]);
    assert_eq!(s.serve_next(&mut inbox).expect("queued").status, Err(E_TIMEDOUT));
    (s.fake.borrow_mut().hold, s.writes) = (hold, writes);
    let sent = register(s.device.regs, sq_tail_doorbell(1, 0));
    let k = s.serve_next(&mut inbox).expect("queued");
    let after = k.at - (t0 + into);
    (k, after, sent)
}

#[test]
fn a_kernel_read_queued_in_another_senders_stall_is_answered_in_time_or_refused() {
    // Queued 5 s into the installer's wait; it takes 5 s.
    let (k, after, sent) = behind_a_stall(0, 5_000, 50, false);
    let refused = (k.status, k.sent) == (Err(E_TIMEDOUT), sent);
    assert!(after <= KERNEL_WAIT_MS || refused, "{:?} {after} ms after it was queued", k.status);
}

#[test]
fn a_kernel_write_given_up_on_in_another_senders_stall_is_not_sent() {
    // RDY 5 s behind each way, queued 1 s into the wait: the kernel stops
    // waiting before the recovery ends.
    let (k, after, sent) = behind_a_stall(50, 1_000, 0, true);
    assert_eq!((k.status, k.sent), (Err(E_TIMEDOUT), sent), "answered {after} ms after queued");
}
