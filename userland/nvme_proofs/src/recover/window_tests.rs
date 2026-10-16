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

//! The retry window counts from the request's start, not from each try: a
//! retry waits a whole 30 s of its own, which ends by 33 s only if it went
//! out within 3 s of the start. Each request's time is its own.

use super::server::Server;
use crate::fake::up;
use crate::protocol::device_status;

/// The driver's answer deadline: the kernel's 35 s, less 2 s for the reply.
const ANSWER_BY_MS: u64 = 33_000;
/// Internal Error (SCT 0, SC 06h): transient, worth a retry.
const INTERNAL: u16 = 0x06 << 1;

#[test]
fn the_retry_window_counts_from_the_requests_start() {
    let mut s = Server::new(up());
    // Commands 1 and 2 fail transiently 2 s apart and 3 would stall: 2 goes
    // out at 2 s, inside the window; 3 would go at 4 s and answer at 34 s.
    s.fake.borrow_mut().hold = 20;
    s.fake.borrow_mut().fail = vec![(1, INTERNAL), (2, INTERNAL)];
    s.fake.borrow_mut().stall.push(3);
    let r = s.read(None);
    assert_eq!((r.status, r.sent), (Err(device_status(0, 0x06)), 2));
    assert!(r.ms <= ANSWER_BY_MS, "answered at {} ms", r.ms);
}

#[test]
fn each_request_gets_its_own_time_whatever_the_last_one_had() {
    let mut s = Server::new(up());
    assert_eq!(s.read(None).status, Ok(()));
    // Ten seconds go by, and the next command takes 29 s: its own 30 s,
    // not what was left of the last request's.
    (0..100).for_each(|_| {
        nonos_libc::mk_uptime_ms();
    });
    s.fake.borrow_mut().hold = 290;
    let r = s.read(None);
    assert_eq!(r.status, Ok(()), "cut short at {} ms", r.ms);
}
