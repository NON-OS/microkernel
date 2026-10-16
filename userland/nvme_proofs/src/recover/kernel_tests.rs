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

//! The kernel's own requests through a long recovery. It sends one at a
//! time and stops waiting 35 s after queueing it; then its next caller
//! queues the next, so a recovery of minutes finds several waiting. The
//! first is bounded by the reply before the recovery, and each one refused
//! for a passed bound hands the next a bound 35 s on (nvm::bound). Each is
//! answered within the 35 s the kernel waits for it, or refused with
//! nothing sent.

use super::after::after_a_recovery;
use super::inbox::kernel;
use crate::protocol::E_TIMEDOUT;

/// How long the kernel waits for a reply (SLOW_BUDGET_MS).
const KERNEL_WAIT_MS: u64 = 35_000;

#[test]
fn the_kernels_requests_through_a_long_recovery_are_answered_in_time_or_refused() {
    // Recoveries of 46 s and of nearly 6 minutes; the request that gets a
    // command stalls, or takes 29.3 s and would then succeed.
    for (slow, stalls) in [(200, true), (200, false), (1270, false)] {
        let (mut s, first) = after_a_recovery(slow);
        if stalls {
            s.fake.borrow_mut().stall.push(2);
        } else {
            s.fake.borrow_mut().hold = 293;
        }
        let end = s.fake.borrow().ticks * 100;
        let queued: Vec<u64> =
            (0..).map(|k| first.at + k * KERNEL_WAIT_MS).take_while(|&at| at <= end).collect();
        let (mut inbox, mut sent) = (kernel(queued.clone()), 0);
        for at in queued {
            let r = s.serve_next(&mut inbox).expect("queued");
            let late = r.at - at > KERNEL_WAIT_MS;
            let what =
                format!("recovery to {end}: queued at {at}, answered at {} {:?}", r.at, r.status);
            assert!(
                !late || (r.status, r.sent) == (Err(E_TIMEDOUT), sent),
                "{what}, sent {}",
                r.sent
            );
            sent = r.sent;
        }
    }
}

#[test]
fn the_kernels_next_request_gets_all_the_time_it_is_still_waited_for() {
    // A recovery of 46 s: the kernel's second request, queued at 35 s, is
    // still waited for until 70 s, and is bounded at 68 s. Its command takes
    // 21 s from its taking at 46 s, and completes inside that.
    let (mut s, first) = after_a_recovery(200);
    s.fake.borrow_mut().hold = 210;
    let mut inbox = kernel([first.at, first.at + KERNEL_WAIT_MS]);
    let refused = s.serve_next(&mut inbox).expect("queued");
    let served = s.serve_next(&mut inbox).expect("queued");
    assert_eq!((refused.status, refused.sent, served.status), (Err(E_TIMEDOUT), 0, Ok(())));
    assert!(served.at - first.at <= 2 * KERNEL_WAIT_MS, "answered at {}", served.at - first.at);
}
