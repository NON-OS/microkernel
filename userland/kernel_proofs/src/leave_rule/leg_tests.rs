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
//! driver's lock taken without waiting and held to the end, each request
//! posted once, its reply picked out by id, a full inbox tried again, and a
//! driver gone or restarted since the post ending the leg.

use core::sync::atomic::{AtomicBool, AtomicU64};

use spin::Mutex;

use super::leg::{Leg, Wire};
use super::rule::Poll;
use crate::services::lifecycle::transport::{DecodedResponse, TransportError};
use crate::services::lifecycle::transport_nowait::{posted, refuse_next, reply};
use crate::services::lifecycle::CapsuleState;

/// A reply as the stand-in decodes it: the request id, then the status.
fn answer(id: u32, status: i32) -> Vec<u8> {
    [id.to_le_bytes(), status.to_le_bytes()].concat()
}

fn decode(data: &[u8]) -> Option<DecodedResponse<'_>> {
    let id = u32::from_le_bytes(data.get(0..4)?.try_into().ok()?);
    let status = i32::from_le_bytes(data.get(4..8)?.try_into().ok()?);
    Some(DecodedResponse { op: 0, request_id: id, status, body: &[] })
}

struct Driver {
    lock: &'static Mutex<()>,
    state: &'static CapsuleState,
}

fn driver() -> Driver {
    let state = CapsuleState { alive: AtomicBool::new(true), generation: AtomicU64::new(1) };
    Driver { lock: Box::leak(Box::new(Mutex::new(()))), state: Box::leak(Box::new(state)) }
}

/// A leg for `ids`, each request its own id's bytes.
fn leg(d: &Driver, ids: &[u32]) -> Leg {
    let wire =
        Wire { lock: d.lock, sender: "kernel", reply_inbox: "reply", state: d.state, decode };
    let requests = ids.iter().map(|&id| (id, id.to_le_bytes().to_vec())).collect();
    Leg::new("driver.test0", 1_000, wire, requests).expect("a running driver")
}

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

#[test]
fn a_post_the_inbox_had_no_room_for_is_tried_again() {
    let d = driver();
    let mut leg = leg(&d, &[7]);
    refuse_next(TransportError::TransportFailure);
    assert_eq!(leg.poll(), Poll::Waiting, "a full inbox ended the leg");
    assert!(posted().is_empty());
    assert_eq!(leg.poll(), Poll::Waiting);
    reply(answer(7, 0));
    assert_eq!(leg.poll(), Poll::Answered(0));
}

#[test]
fn a_driver_gone_ends_the_leg() {
    let d = driver();
    let mut leg = leg(&d, &[7]);
    refuse_next(TransportError::Dead);
    assert_eq!(leg.poll(), Poll::Gone);
}

#[test]
fn a_driver_restarted_since_the_post_ends_the_leg() {
    let d = driver();
    let mut leg = leg(&d, &[7]);
    assert_eq!(leg.poll(), Poll::Waiting);
    d.state.restart();
    reply(answer(7, 0));
    assert_eq!(leg.poll(), Poll::Gone, "a reply from the new generation answered the old post");
}

#[test]
fn the_lock_is_held_until_the_leg_drops() {
    let d = driver();
    let mut leg = leg(&d, &[7]);
    assert_eq!(leg.poll(), Poll::Waiting);
    assert!(d.lock.try_lock().is_none(), "another caller could send behind the leg");
    drop(leg);
    assert!(d.lock.try_lock().is_some());
}
