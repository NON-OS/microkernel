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

//! The late completion of a transfer given up is not the answer to the TRB
//! written at its address one lap round the ring.

use super::command_at::{endpoint, quiet_context, ring_base};
use super::events::{command, transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, SLOT, TRB};
use crate::constants::COMMAND_RING_TRBS;
use crate::controller::{bulk_transfer, recover_endpoint};

#[test]
fn a_transfer_one_lap_later_is_not_answered_by_a_stale_event() {
    // Before the fix: the late completion of a transfer that timed out sat
    // parked, and the TRB written at the same address one lap round the
    // 64-TRB ring was taken as complete at once, with the old residual.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let mut pipes = fx.bulk_pipes();
    let base = ring_base(&cmd);
    let first = pipes.in_ring.enqueue_phys();
    fx.hc.post(transfer(first, CC_SUCCESS, 13, SLOT, BULK_IN_DCI));
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    let out = quiet_context(&fx);
    let ep = endpoint(&pipes.in_ring, &out, BULK_IN_DCI);
    recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, ep).expect("recovered");
    // Every other TRB of the lap completes normally.
    for _ in 0..COMMAND_RING_TRBS - 1 {
        let at = pipes.in_ring.enqueue_phys();
        if at == first {
            break;
        }
        fx.hc.post(transfer(at, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
        let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
        assert_eq!(moved, Ok(512));
    }
    assert_eq!(pipes.in_ring.enqueue_phys(), first, "one lap round");
    fx.hc.post(transfer(first, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(512), "its own event, not the stale residual of 13");
}
