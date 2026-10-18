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

//! A request queued after the inbox was seen empty was sent fresh, however
//! soon after a recovery: the recovery's bound is dropped as the server
//! finds nothing waiting (server::take), and the request has its own time;
//! so has the kernel's next once its request spent the bound. Kept, the
//! bound refused such a request or cut its wait short, and the timeout that
//! followed sent an Abort to a healthy controller.

use super::after::after_a_recovery;
use super::inbox::kernel;

#[test]
fn a_fresh_request_a_minute_after_a_recovery_is_served() {
    let (mut s, first) = after_a_recovery(0);
    let fresh = s.serve_next(&mut kernel([first.at + 60_000])).expect("queued");
    assert_eq!((fresh.status, fresh.sent), (Ok(()), 1), "refused for an old bound");
}

#[test]
fn a_fresh_request_25_s_after_a_recovery_gets_its_own_30_s() {
    let (mut s, first) = after_a_recovery(0);
    s.fake.borrow_mut().hold = 50; // 5 s, inside its own 30 s
    let from = s.fake.borrow().admin_seen.len();
    let fresh = s.serve_next(&mut kernel([first.at + 25_000])).expect("queued");
    assert_eq!((fresh.status, s.ops_since(from)), (Ok(()), vec![]), "cut short at {}", fresh.ms);
}

#[test]
fn a_request_queued_half_a_second_after_the_inbox_was_seen_empty_is_fresh() {
    // Nothing waits as the recovery ends, so a 1 ms look finds the inbox
    // empty; one queued 0.5 s on has its own 29.3 s, not what the bound left.
    let (mut s, first) = after_a_recovery(0);
    let end = s.fake.borrow().ticks * 100;
    s.fake.borrow_mut().hold = 293;
    let fresh = s.serve_next(&mut kernel([end + 500])).expect("queued");
    assert_eq!(fresh.status, Ok(()), "cut at {} ms after the last reply", fresh.at - first.at);
}

#[test]
fn the_kernels_request_after_one_that_spent_the_bound_has_its_own_time() {
    // N+1 is served, and the kernel queues N+2 as the reply reaches it,
    // before the inbox is looked at: N+2's 29.3 s is its own.
    let (mut s, first) = after_a_recovery(0);
    let mut inbox = kernel([first.at]);
    let served = s.serve_next(&mut inbox).expect("queued");
    inbox.push_back((served.at, 0));
    s.fake.borrow_mut().hold = 293;
    let next = s.serve_next(&mut inbox).expect("queued");
    assert_eq!((served.status, next.status), (Ok(()), Ok(())), "N+2 cut at {} ms", next.ms);
}
