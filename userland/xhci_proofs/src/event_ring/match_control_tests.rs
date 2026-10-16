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

//! Which event completes a control transfer: the Transfer Event whose
//! pointer, slot and endpoint name its last TRB, or an error on one of its
//! earlier TRBs.

use super::events::{command, transfer, CC_STALL, CC_SUCCESS, CC_TRB_ERROR};
use super::fixture::{Fixture, EP0_DCI, SLOT, TRB};
use super::match_impostors::not_configured;
use crate::controller::{get_device_descriptor, DEVICE_DESCRIPTOR_LEN};

#[test]
fn a_transfer_event_for_another_trb_does_not_complete_a_control_transfer() {
    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let setup = ep0.enqueue_phys();
    let status = setup + 2 * TRB;
    let out = fx.pool.alloc(64).expect("buffer");
    // None of these is one of the transfer's own three TRBs.
    for p in [status + TRB, status ^ (1 << 40), 0, setup.wrapping_sub(TRB)] {
        fx.hc.post(transfer(p, CC_STALL, 0, SLOT, EP0_DCI));
    }
    fx.hc.post(command(status, CC_TRB_ERROR, SLOT));
    fx.hc.post(transfer(status, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    assert_eq!(got, Ok(DEVICE_DESCRIPTOR_LEN as usize));
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn a_transfer_event_for_a_slot_or_endpoint_never_configured_does_not_complete_a_control_transfer() {
    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let status = ep0.enqueue_phys() + 2 * TRB;
    let out = fx.pool.alloc(64).expect("buffer");
    for (slot, ep) in not_configured(EP0_DCI) {
        fx.hc.post(transfer(status, CC_STALL, 0, slot, ep));
    }
    fx.hc.post(transfer(status, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    assert_eq!(got, Ok(DEVICE_DESCRIPTOR_LEN as usize));
    assert_eq!(fx.hc.pending(), 0);
}
