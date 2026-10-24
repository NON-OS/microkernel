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

//! One leg's round (src/hardware/block_device/leave/leg_poll.rs): the
//! driver's lock taken without waiting, each request posted once and its
//! reply picked out by id, the requests in turn.

use super::leg_rig::{answer, driver, leg};
use super::rule::Poll;
use crate::services::lifecycle::transport_nowait::{posted, reply};

#[test]
fn a_held_lock_waits_and_posts_nothing() {
    let d = driver();
    let mut leg = leg(&d, &[7]);
    let held = d.lock.lock();
    assert_eq!(leg.poll(), Poll::Waiting);
    assert!(posted().is_empty(), "posted behind another caller's request");
    drop(held);
    assert_eq!(leg.poll(), Poll::Waiting);
    assert_eq!(posted().len(), 1);
}

#[test]
fn each_request_is_posted_once_and_answered_by_its_reply() {
    let d = driver();
    let mut leg = leg(&d, &[7]);
    assert_eq!(leg.poll(), Poll::Waiting);
    assert_eq!(leg.poll(), Poll::Waiting);
    assert_eq!(posted(), [7u32.to_le_bytes().to_vec()], "posted again while waiting");
    reply(answer(7, 0));
    assert_eq!(leg.poll(), Poll::Answered(0));
}

#[test]
fn a_reply_to_another_request_is_dropped() {
    let d = driver();
    let mut leg = leg(&d, &[7]);
    assert_eq!(leg.poll(), Poll::Waiting);
    reply(answer(6, -5));
    reply(answer(7, 0));
    assert_eq!(leg.poll(), Poll::Answered(0), "an older request's reply was taken as this one's");
}

#[test]
fn requests_go_in_turn_and_the_first_failure_is_the_answer() {
    let d = driver();
    let mut leg = leg(&d, &[7, 8]);
    assert_eq!(leg.poll(), Poll::Waiting);
    reply(answer(7, -5));
    assert_eq!(leg.poll(), Poll::Waiting, "the second still to go");
    reply(answer(8, 0));
    assert_eq!(leg.poll(), Poll::Answered(-5));
    assert_eq!(posted(), [7u32.to_le_bytes().to_vec(), 8u32.to_le_bytes().to_vec()]);
}
