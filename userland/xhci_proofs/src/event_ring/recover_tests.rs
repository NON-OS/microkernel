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

//! Bringing an endpoint back after a failed transfer, against the producer
//! written from the specification. A stuck command is `abort_ring_tests`.

use super::command_at::{command_at, endpoint, quiet_context, ring_base};
use super::command_at::{RESET_ENDPOINT, SET_TR_DEQUEUE, STOP_ENDPOINT};
use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT, TRB};
use crate::controller::command::recovered::CC_CONTEXT_STATE_ERROR;
use crate::controller::recover_endpoint;

#[test]
fn a_halted_endpoint_is_reset_and_moved_past_its_ring() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let (ring, out) = (fx.transfer_ring(), quiet_context(&fx));
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    let ep = endpoint(&ring, &out, INT_DCI);
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, ep).expect("recovered");
    let reset = command_at(base);
    assert_eq!(reset.get_type(), RESET_ENDPOINT);
    assert_eq!(reset.endpoint_id(), INT_DCI);
    assert_eq!(reset.slot_id(), SLOT);
    let set = command_at(base + TRB);
    assert_eq!(set.get_type(), SET_TR_DEQUEUE);
    assert_eq!(set.get_pointer(), ring.enqueue_phys() | ring.cycle() as u64);
}

#[test]
fn an_endpoint_still_running_is_stopped_instead() {
    // A transfer that timed out leaves the endpoint Running: Reset Endpoint
    // answers Context State Error, and Stop Endpoint takes its place.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let (ring, out) = (fx.transfer_ring(), quiet_context(&fx));
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_CONTEXT_STATE_ERROR, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + 2 * TRB, CC_SUCCESS, SLOT));
    let ep = endpoint(&ring, &out, 1);
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, ep).expect("recovered");
    assert_eq!(command_at(base + TRB).get_type(), STOP_ENDPOINT);
    assert_eq!(command_at(base + 2 * TRB).get_type(), SET_TR_DEQUEUE);
}
