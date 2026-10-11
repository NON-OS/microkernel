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

//! The polled bulk IN the USB network drivers read with: it arms one TRB,
//! answers at once while nothing has come, takes only the completion that
//! names its TRB, and leaves every other event for its own taker.

use super::events::{command, transfer, CC_STALL, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, INT_DCI, SLOT, TRB};
use crate::controller::{poll_bulk_in, BulkPoll};
use crate::rings::event::IssuedTransfer;

pub(super) const LEN: u32 = 1536;

#[test]
fn nothing_come_answers_at_once_and_arms_one_trb() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let first = pipes.in_ring.enqueue_phys();
    for _ in 0..3 {
        let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
        assert_eq!(got, Ok(BulkPoll::Pending));
    }
    assert_eq!(pipes.armed, Some((first, LEN)), "armed once, on the IN ring");
    assert_eq!(pipes.in_ring.enqueue_phys(), first + TRB, "one TRB for three looks");
    assert_eq!(fx.doorbells.wrote32(SLOT as usize * 4), BULK_IN_DCI as u32);
    assert_ne!(pipes.poll_buf.phys(), pipes.buf.phys(), "the poll has its own page");
}

#[test]
fn the_completion_of_the_armed_trb_hands_up_the_bytes_less_the_residual() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    for r in [0, 1, 100, LEN - 1, LEN, LEN + 1, 0xFF_FFFF] {
        let issued = pipes.in_ring.enqueue_phys();
        fx.hc.post(transfer(issued, CC_SUCCESS, r, SLOT, BULK_IN_DCI));
        let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
        assert_eq!(got, Ok(BulkPoll::Complete(LEN - r.min(LEN))), "residual {r:#x}");
        assert_eq!(pipes.armed, None, "the TRB is spent");
    }
}

#[test]
fn other_events_are_left_for_their_takers() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    // A command's completion stays on the ring for the command wait.
    fx.hc.post(command(issued, CC_SUCCESS, 1));
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Pending));
    assert!(fx.ring.has_event(), "the command completion was taken");
    fx.ring.advance();
    // Another endpoint's completion naming the same address is parked.
    fx.hc.post(transfer(issued, CC_STALL, 0, SLOT, INT_DCI));
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Pending));
    let other = IssuedTransfer { phys: issued, slot: SLOT, dci: INT_DCI };
    assert!(fx.ring.take_parked(other).is_some(), "the interrupt endpoint's event was lost");
}
