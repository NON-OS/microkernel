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

//! Another sender's read (an installer's) ahead of the kernel's after a
//! recovery, both queued as the reply before it went. The bound is the
//! kernel's: the other read is held to it and leaves it, and one that
//! stalls keeps the sooner, so the kernel's is answered in its 35 s or
//! refused, nothing sent. The other read spent or passed on the bound, and
//! the kernel's read, a write as easily, was Ok after it was reported lost.

use super::after::after_a_recovery;
use super::inbox::{Inbox, Reply};
use crate::fake::{register, Fake};
use crate::nvm::sq_tail_doorbell;
use crate::protocol::E_TIMEDOUT;

/// How long the kernel waits for a reply (SLOW_BUDGET_MS).
const KERNEL_WAIT_MS: u64 = 35_000;
const INSTALLER: u32 = 42;

/// What a test sets the fake to before a read is taken.
type Set = fn(&mut Fake);

/// The other read, then the kernel's, after a recovery of `slow`, `theirs`
/// and `kernels` setting the fake before each: the kernel's reply, the ms
/// from its queueing to it, and the I/O commands sent before it was taken.
fn behind_another(slow: u64, theirs: Set, kernels: Set) -> (Reply, u64, u32) {
    let (mut s, first) = after_a_recovery(slow);
    let mut inbox = Inbox::from([(first.at, INSTALLER), (first.at, 0)]);
    theirs(&mut s.fake.borrow_mut());
    s.serve_next(&mut inbox).expect("queued");
    kernels(&mut s.fake.borrow_mut());
    let sent = register(s.device.regs, sq_tail_doorbell(1, 0));
    let r = s.serve_next(&mut inbox).expect("queued");
    let after = r.at - first.at;
    (r, after, sent)
}

#[test]
fn the_kernels_read_behind_one_served_keeps_the_bound() {
    // Recovery 5.7 s; the other read is served, the kernel's stalls.
    let (r, after, _) = behind_another(0, |_| (), |f| f.stall.push(3));
    assert_eq!(r.status, Err(E_TIMEDOUT));
    assert!(after <= KERNEL_WAIT_MS, "answered {after} ms after queued");
}

#[test]
fn the_kernels_read_behind_one_refused_is_refused_too() {
    // Recovery 34.5 s, past the bound: the other read is refused first.
    let (r, after, sent) = behind_another(143, |_| (), |f| f.hold = 293);
    let refused = (r.status, r.sent) == (Err(E_TIMEDOUT), sent);
    assert!(refused, "{:?}, answered at {after} ms", r.status);
}

#[test]
fn the_kernels_read_behind_one_timed_out_keeps_the_sooner_bound() {
    // Recovery 5.7 s; the other read stalls to the bound, a recovery again.
    let stalls = |f: &mut Fake| f.stall.push(2);
    let (r, after, sent) = behind_another(0, stalls, |f| f.hold = 100);
    let refused = (r.status, r.sent) == (Err(E_TIMEDOUT), sent);
    assert!(refused, "{:?}, answered at {after} ms", r.status);
}
