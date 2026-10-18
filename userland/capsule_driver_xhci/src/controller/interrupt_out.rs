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
//! One report to an interrupt OUT endpoint: copied into its buffer, sent
//! on one Normal TRB, and waited for. A security key reads its requests
//! here (CTAPHID); a report is at most the endpoint's own payload.

use super::transfer::bulk::wait::wait_bulk;
use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS};
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::slots::SlotResources;
use crate::trb::builders::normal::normal;

/// The bytes the device took.
pub fn interrupt_out(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    res: &mut SlotResources,
    dci: u8,
    data: &[u8],
) -> XhciResult<u16> {
    let slot = res.slot_id;
    let ep = res.interrupt_mut(dci).ok_or(XhciError::ControllerUnsupported)?;
    if !dci.is_multiple_of(2) || data.is_empty() || data.len() > ep.len as usize {
        return Err(XhciError::ControllerUnsupported);
    }
    let dst = ep.buf.as_mut_ptr::<u8>();
    for (i, &b) in data.iter().enumerate() {
        // SAFETY: the buffer holds `ep.len` bytes and `i` is below that.
        unsafe { core::ptr::write_volatile(dst.add(i), b) };
    }
    let len = data.len() as u32;
    let issued = ep.ring.enqueue(normal(ep.buf.phys(), len, ep.ring.cycle() != 0, true, false))?;
    ring_doorbell(doorbell_base, slot, dci);
    let event = wait_bulk(intr_base, IssuedTransfer { phys: issued, slot, dci }, evt_ring)?;
    match event.completion_code() {
        CC_SUCCESS | CC_SHORT_PACKET => {
            ep.ring.retire();
            Ok((len - (event.d2 & 0x00FF_FFFF).min(len)) as u16)
        }
        cc => Err(XhciError::TransferCompletionFailed(cc)),
    }
}
