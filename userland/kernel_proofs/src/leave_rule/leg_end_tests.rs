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

//! How a leg's round ends (src/hardware/block_device/leave/leg_poll.rs): a
//! full inbox tried again, a driver gone or restarted since the post ending
//! the leg, and the lock held until the leg drops.

use super::leg_rig::{answer, driver, leg};
use super::rule::Poll;
use crate::services::lifecycle::transport::TransportError;
use crate::services::lifecycle::transport_nowait::{posted, refuse_next, reply};

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
