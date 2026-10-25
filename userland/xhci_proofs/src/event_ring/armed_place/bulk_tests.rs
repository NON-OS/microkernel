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

use super::super::bulk_poll_tests::LEN;
use super::super::events::{command, transfer, CC_SUCCESS};
use super::super::fixture::{Fixture, BULK_IN_DCI, SLOT};
use super::take_every_place;
use crate::controller::{poll_bulk_in, BulkPoll};
use crate::error::XhciError;

#[test]
fn a_command_completion_nobody_waits_for_does_not_hold_up_the_poll() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let armed = pipes.in_ring.enqueue_phys();
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Pending));
    fx.hc.post(command(0xF0_DEAD_0000, CC_SUCCESS, SLOT));
    fx.hc.post(transfer(armed, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!((got, fx.hc.pending()), (Ok(BulkPoll::Complete(LEN)), 0));
}

#[test]
fn with_every_place_taken_the_poll_arms_nothing() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    take_every_place(&mut fx);
    let at = pipes.in_ring.enqueue_phys();
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, LEN);
    assert_eq!(got, Err(XhciError::TooManyArmed));
    assert_eq!((pipes.armed, pipes.in_ring.enqueue_phys()), (None, at), "no TRB queued");
}
