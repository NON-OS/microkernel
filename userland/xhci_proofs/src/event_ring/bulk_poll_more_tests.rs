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

//! The polled bulk IN beside the rest of the slot's life: a waited OUT
//! between two looks, a STALL, and a length the pipe cannot carry.

use super::bulk_poll_tests::LEN;
use super::events::{transfer, CC_STALL, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, BULK_OUT_DCI, SLOT, TRB};
use crate::controller::{bulk_transfer, poll_bulk_in, BulkPoll};
use crate::error::XhciError;
use crate::protocol::BULK_MAX;

#[test]
fn a_waited_out_between_two_looks_keeps_the_polls_completion() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let armed = pipes.in_ring.enqueue_phys();
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Pending));
    // The frame comes in while a frame goes out; the OUT's wait meets the
    // IN's completion first and parks it.
    fx.hc.post(transfer(armed, CC_SUCCESS, 36, SLOT, BULK_IN_DCI));
    fx.hc.post(transfer(pipes.out_ring.enqueue_phys(), CC_SUCCESS, 0, SLOT, BULK_OUT_DCI));
    let sent = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, false, 60);
    assert_eq!(sent, Ok(60));
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Complete(LEN - 36)));
}

#[test]
fn a_stall_is_reported_and_the_next_look_arms_afresh() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let first = pipes.in_ring.enqueue_phys();
    fx.hc.post(transfer(first, CC_STALL, 0, SLOT, BULK_IN_DCI));
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Err(XhciError::TransferCompletionFailed(CC_STALL)));
    assert_eq!(pipes.armed, None);
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Pending));
    assert_eq!(pipes.armed, Some((first + TRB, LEN)), "a fresh TRB, not the stalled one");
}

#[test]
fn a_length_outside_one_to_bulk_max_arms_nothing() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let at = pipes.in_ring.enqueue_phys();
    for len in [0, BULK_MAX as u32 + 1, u32::MAX] {
        let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, len);
        assert_eq!(got, Err(XhciError::ControllerUnsupported), "length {len}");
    }
    assert_eq!((pipes.armed, pipes.in_ring.enqueue_phys()), (None, at));
}
