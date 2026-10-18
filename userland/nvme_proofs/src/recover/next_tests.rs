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

//! The request taken after a recovery, already queued when it is. The
//! kernel queues its next request as a reply reaches it and waits 35 s for
//! it, while the recovery after that reply runs on the same clock. So that
//! request is answered within 33 s of the reply: its waits get what is
//! left, and with nothing left it is refused with nothing sent. One queued
//! after the inbox was seen empty is fresh (fresh_tests).

use super::after::after_a_recovery;
use super::inbox::{kernel, Inbox};
use crate::protocol::E_TIMEDOUT;

/// The driver's answer deadline: the kernel's 35 s, less 2 s for the reply.
const ANSWER_BY_MS: u64 = 33_000;
/// One read of the fake's 100 ms clock lies between a timed-out wait's end
/// and its reply: the read that bounds the request after it.
const BOUND_READ_MS: u64 = 100;

#[test]
fn the_next_request_is_answered_within_33_s_of_the_reply_before_the_recovery() {
    // Its command stalls; or it takes 29.3 s and would then succeed, after
    // the kernel stopped waiting.
    for stalls in [true, false] {
        let (mut s, first) = after_a_recovery(0);
        if stalls {
            s.fake.borrow_mut().stall.push(2);
        } else {
            s.fake.borrow_mut().hold = 293;
        }
        let next = s.serve_next(&mut kernel([first.at])).expect("queued");
        assert_eq!(next.status, Err(E_TIMEDOUT), "not Ok after the kernel gave up");
        let after = next.at - first.at;
        assert!(after <= ANSWER_BY_MS + BOUND_READ_MS, "answered {after} ms after the last reply");
    }
}

#[test]
fn a_request_that_waited_out_a_long_recovery_is_refused_and_a_fresh_one_served() {
    // RDY 127 s behind CC.EN each way: a recovery of over 4 minutes.
    let (mut s, first) = after_a_recovery(1270);
    let mut inbox = kernel([first.at]);
    let next = s.serve_next(&mut inbox).expect("queued");
    assert!(next.at - first.at > 250_000, "a recovery of {} ms", next.at - first.at);
    let refused = (next.status, next.sent, next.ms <= 100);
    assert_eq!(refused, (Err(E_TIMEDOUT), 0, true), "refused at once, nothing sent");
    // Queued 10 s after the inbox is next seen empty: fresh.
    inbox.push_back((next.at + 10_000, 0));
    let fresh = s.serve_next(&mut inbox).expect("queued");
    assert_eq!((fresh.status, fresh.sent), (Ok(()), 1), "served as any other");
}

#[test]
fn another_senders_read_that_waited_out_a_long_recovery_is_refused_too() {
    // Held to the kernel's bound: its own call waits 65 s from a send the
    // driver cannot see, and the recovery took over 4 minutes.
    let (mut s, first) = after_a_recovery(1270);
    let theirs = s.serve_next(&mut Inbox::from([(first.at, 42)])).expect("queued");
    assert_eq!((theirs.status, theirs.sent), (Err(E_TIMEDOUT), 0), "served after its sender left");
}
