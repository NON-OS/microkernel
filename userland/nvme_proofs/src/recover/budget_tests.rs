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

//! A read, write or flush is answered inside the kernel's reply wait, and
//! only then is its command recovered from. The kernel stops waiting at
//! 35 s (src/services/lifecycle/reply_wait.rs); recovery run first, after a
//! 30 s timeout, answered at 35.8 s on a controller whose Abort went
//! unanswered, and a late retry at 50 s.

use super::server::Server;
use crate::fake::up;
use crate::protocol::{device_status, E_TIMEDOUT};

/// The driver's own answer deadline: the kernel's 35 s, less 2 s for the
/// reply to reach it.
const ANSWER_BY_MS: u64 = 33_000;
/// Internal Error (SCT 0, SC 06h): transient, worth a retry.
const INTERNAL: u16 = 0x06 << 1;

#[test]
fn a_timeout_is_answered_in_time_and_recovered_before_the_next_request() {
    let mut s = Server::new(up());
    s.fake.borrow_mut().stall.push(1);
    s.fake.borrow_mut().mute = true; // the Abort, too, goes unanswered
    let from = s.fake.borrow().admin_seen.len();
    let r = s.read(None);
    assert_eq!(r.status, Err(E_TIMEDOUT));
    assert!((30_000..=ANSWER_BY_MS).contains(&r.ms), "answered at {} ms", r.ms);
    assert_eq!(r.admin, from, "answered before the Abort went out");
    assert_eq!(s.ops_since(from), [0x08, 0x05, 0x01], "then reset");
    let next = s.read(None);
    assert_eq!((next.status, next.admin), (Ok(()), from + 3), "served once recovered");
}

#[test]
fn a_transient_failure_late_in_the_request_is_answered_not_sent_again() {
    let mut s = Server::new(up());
    // Command 1 fails transiently after 25 s: a retry's own 30 s would
    // answer at 55 s.
    (s.fake.borrow_mut().hold, s.fake.borrow_mut().fail) = (250, vec![(1, INTERNAL)]);
    let r = s.read(None);
    assert_eq!((r.status, r.sent), (Err(device_status(0, 0x06)), 1));
    assert!(r.ms <= ANSWER_BY_MS, "answered at {} ms", r.ms);
}

#[test]
fn a_transient_failure_early_is_sent_again_and_still_answered_in_time() {
    let mut s = Server::new(up());
    (s.fake.borrow_mut().hold, s.fake.borrow_mut().fail) = (20, vec![(1, INTERNAL)]);
    s.fake.borrow_mut().stall.push(2);
    let from = s.fake.borrow().admin_seen.len();
    let r = s.read(None);
    assert_eq!((r.status, r.sent, r.admin), (Err(E_TIMEDOUT), 2, from));
    assert!((32_000..=ANSWER_BY_MS).contains(&r.ms), "answered at {} ms", r.ms);
    assert_eq!(s.ops_since(from), [0x08, 0x05, 0x01], "command 2 recovered after");
}
