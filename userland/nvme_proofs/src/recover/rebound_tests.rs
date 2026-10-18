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

//! What does not end a bound. A recovery starts one of its own, which no
//! sender the last bound served ends: that sender's next request may sit
//! ahead of a kernel request queued while the new stall ran. Nor does a
//! refused request's sender, which may have sent it again while the
//! recovery ran, its own call timed out, ahead of later kernel requests.

use super::after::after_a_recovery;
use super::inbox::Inbox;
use crate::fake::register;
use crate::nvm::sq_tail_doorbell;
use crate::protocol::E_TIMEDOUT;

/// How long the kernel waits for a reply (SLOW_BUDGET_MS).
const KERNEL_WAIT_MS: u64 = 35_000;
const INSTALLER: u32 = 42;
const OTHER: u32 = 43;

#[test]
fn a_sender_the_last_bound_served_does_not_end_the_next_recoverys() {
    // After the kernel's recovery the installer is served, and queues its
    // next; another sender's read then stalls, and 1 s into that wait the
    // kernel queues a read that takes 5 s. The installer's next goes first.
    let (mut s, first) = after_a_recovery(0);
    let mut inbox = Inbox::from([(first.at, INSTALLER), (first.at, OTHER)]);
    let theirs = s.serve_next(&mut inbox).expect("queued");
    inbox.extend([(theirs.at, INSTALLER), (theirs.at + 1_000, 0)]);
    s.fake.borrow_mut().stall.push(3);
    assert_eq!(s.serve_next(&mut inbox).expect("queued").status, Err(E_TIMEDOUT));
    s.fake.borrow_mut().hold = 50;
    s.serve_next(&mut inbox).expect("queued");
    let sent = register(s.device.regs, sq_tail_doorbell(1, 0));
    let k = s.serve_next(&mut inbox).expect("queued");
    let after = k.at - (theirs.at + 1_000);
    let refused = (k.status, k.sent) == (Err(E_TIMEDOUT), sent);
    assert!(after <= KERNEL_WAIT_MS || refused, "{:?} {after} ms after it was queued", k.status);
}

#[test]
fn a_refused_senders_resend_ahead_of_the_kernels_does_not_end_the_bound() {
    // A recovery of nearly 6 minutes: the installer's read and the kernel's
    // first two are refused; the installer sent its read again 65 s in,
    // ahead of the kernel's third, which would take 5 s.
    let (mut s, first) = after_a_recovery(1270);
    let at = [0, 0, 35_000, 65_000, 70_000].map(|ms| first.at + ms);
    let pids = [INSTALLER, 0, 0, INSTALLER, 0];
    let mut inbox: Inbox = at.into_iter().zip(pids).collect();
    s.fake.borrow_mut().hold = 50;
    for (queued, pid) in at.into_iter().zip(pids) {
        let sent = register(s.device.regs, sq_tail_doorbell(1, 0));
        let r = s.serve_next(&mut inbox).expect("queued");
        let refused = (r.status, r.sent) == (Err(E_TIMEDOUT), sent);
        let late = r.at - queued > KERNEL_WAIT_MS;
        assert!(pid != 0 || !late || refused, "{:?} {} ms after queued", r.status, r.at - queued);
    }
}
