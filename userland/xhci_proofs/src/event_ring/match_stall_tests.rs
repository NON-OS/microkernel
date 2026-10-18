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

//! A STALL ends a control transfer at once when it is on one of the
//! transfer's own earlier TRBs, and not when it names another slot or
//! endpoint.

use super::events::{transfer, CC_STALL, CC_SUCCESS};
use super::fixture::{Fixture, EP0_DCI, INT_DCI, SLOT, TRB};
use crate::controller::{get_device_descriptor, DEVICE_DESCRIPTOR_LEN};
use crate::error::XhciError;

#[test]
fn a_stall_on_the_data_stage_ends_the_control_transfer_at_once() {
    /*
     * The controller reports a STALL on the TRB it was on and halts EP0; the
     * status stage is never reached. The wait ends on that event with the
     * STALL, where it used to park it and run to its timeout.
     */
    for stage in [0, 1] {
        let mut fx = Fixture::new();
        let mut ep0 = fx.transfer_ring();
        let at = ep0.enqueue_phys() + stage * TRB;
        let out = fx.pool.alloc(64).expect("buffer");
        fx.hc.post(transfer(at, CC_STALL, 0, SLOT, EP0_DCI));
        let started = std::time::Instant::now();
        let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
        assert_eq!(got, Err(XhciError::TransferCompletionFailed(CC_STALL)), "stage {stage}");
        assert!(started.elapsed() < std::time::Duration::from_millis(500), "it ran to the timeout");
    }
}

#[test]
fn a_stall_on_another_slot_or_endpoint_does_not_end_a_control_transfer() {
    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let setup = ep0.enqueue_phys();
    let out = fx.pool.alloc(64).expect("buffer");
    fx.hc.post(transfer(setup + TRB, CC_STALL, 0, SLOT + 1, EP0_DCI));
    fx.hc.post(transfer(setup + TRB, CC_STALL, 0, SLOT, INT_DCI));
    fx.hc.post(transfer(setup + 2 * TRB, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    assert_eq!(got, Ok(DEVICE_DESCRIPTOR_LEN as usize));
}
