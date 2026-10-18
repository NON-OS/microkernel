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

//! Which event completes a bulk transfer: the Transfer Event whose
//! pointer, slot and endpoint name its TRB.

use super::events::{command, transfer, CC_STALL, CC_SUCCESS, CC_TRB_ERROR};
use super::fixture::{Fixture, BULK_IN_DCI, SLOT};
use super::match_impostors::{not_configured, not_issued};
use crate::controller::bulk_transfer;

#[test]
fn a_transfer_event_for_another_trb_does_not_complete_a_bulk_transfer() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    for p in not_issued(issued) {
        fx.hc.post(transfer(p, CC_STALL, 7, SLOT, BULK_IN_DCI));
    }
    fx.hc.post(command(issued, CC_TRB_ERROR, SLOT));
    fx.hc.post(transfer(issued, CC_SUCCESS, 100, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(412), "the residual of the event that names the TRB");
}

#[test]
fn a_transfer_event_for_a_slot_or_endpoint_never_configured_does_not_complete_a_bulk_transfer() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    for (slot, ep) in not_configured(BULK_IN_DCI) {
        fx.hc.post(transfer(issued, CC_STALL, 7, slot, ep));
    }
    fx.hc.post(transfer(issued, CC_SUCCESS, 100, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(412));
    // The impostors were parked. The next transfer on the ring is not handed
    // one of them either.
    let next = pipes.in_ring.enqueue_phys();
    fx.hc.post(transfer(next, CC_SUCCESS, 12, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(500));
}
