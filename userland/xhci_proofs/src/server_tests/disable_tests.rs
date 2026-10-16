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

//! Disable Slot gives the slot id and its rings' frames back, so nothing
//! kept for the slot may answer a TRB of the next device given that id: a
//! report its poll never collected used to stay parked, to complete a TRB
//! at the same address at once, with stale data.

use super::bench::Bench;
use super::hc_device::lock;
use crate::event_ring::events::{transfer, CC_SUCCESS};
use crate::protocol::{
    E_AGAIN, OP_ALLOC_TRANSFER_RING, OP_DISABLE_SLOT, OP_GET_DEVICE_DESCRIPTOR, OP_INTERRUPT_IN,
};
use crate::rings::event::IssuedTransfer;

const HIGH_SPEED: u32 = 3;
const INT_DCI: u8 = 3;

#[test]
fn a_report_kept_for_a_disabled_slot_is_dropped_with_it() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    assert_eq!(b.call(OP_ALLOC_TRANSFER_RING, &[slot, 0x81, 8, 0, 10, 0]).status, 0);
    assert_eq!(b.call(OP_INTERRUPT_IN, &[slot, INT_DCI, 8, 0]).status, E_AGAIN, "armed");
    let res = b.mux.ctxs[0].driver.slots.resources_mut(slot, 8).expect("addressed");
    let phys = res.interrupt[0].armed.expect("armed");
    // The report lands while a descriptor read waits, which keeps it.
    lock(&b.hc).producer.post(transfer(phys, CC_SUCCESS, 0, slot, INT_DCI));
    lock(&b.hc).answer = |s| vec![0x12; s.length as usize];
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[slot]).status, 0);
    assert_eq!(b.call(OP_DISABLE_SLOT, &[slot]).status, 0);
    let issued = IssuedTransfer { phys, slot, dci: INT_DCI };
    assert!(b.mux.ctxs[0].driver.event_ring.take_parked(issued).is_none());
}
