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

//! A bulk pipe the class driver resets comes back with its data toggle at
//! zero on the host side as well, and recovery says how it came back.

use super::command_at::{command_at, endpoint, quiet_context, ring_base};
use super::command_at::{RESET_ENDPOINT, SET_TR_DEQUEUE};
use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, SLOT, TRB};
use super::toggle_slot::{storage_slot, CONTEXT_SIZE};
use crate::controller::command::recovered::CC_CONTEXT_STATE_ERROR;
use crate::controller::{recover_endpoint, reset_bulk_endpoint, Recovered};

#[test]
fn a_halted_bulk_pipe_is_reset_and_nothing_more() {
    // Reset Endpoint with TSP clear already zeroes the toggle.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let res = storage_slot(&fx);
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    reset_bulk_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, &res, CONTEXT_SIZE, true)
        .expect("reset");
    assert_eq!(command_at(base).get_type(), RESET_ENDPOINT);
    assert_eq!(command_at(base + TRB).get_type(), SET_TR_DEQUEUE);
    assert_eq!(command_at(base + 2 * TRB).get_type(), 0, "no Configure Endpoint");
}

#[test]
fn recovery_says_which_way_the_endpoint_came_back() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let (ring, out) = (fx.transfer_ring(), quiet_context(&fx));
    let base = ring_base(&cmd);
    fx.hc.post(command(base, CC_SUCCESS, SLOT));
    fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
    let got =
        recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, endpoint(&ring, &out, 1));
    assert_eq!(got, Ok(Recovered::Reset));
    fx.hc.post(command(base + 2 * TRB, CC_CONTEXT_STATE_ERROR, SLOT));
    fx.hc.post(command(base + 3 * TRB, CC_CONTEXT_STATE_ERROR, SLOT));
    fx.hc.post(command(base + 4 * TRB, CC_SUCCESS, SLOT));
    let got =
        recover_endpoint(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, endpoint(&ring, &out, 1));
    assert_eq!(got, Ok(Recovered::Stopped), "already Stopped is not Halted either");
}
