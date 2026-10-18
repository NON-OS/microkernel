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

//! One batch of isoch OUT packets: copied into the free half, a TD each,
//! the doorbell rung, then the batch before waited for. The ring always
//! holds the next batch while the controller plays the last, so a client
//! that keeps sending keeps the endpoint fed.

use super::batch::batch_fits;
use super::burst::burst_fields;
use super::pipe::IsochPipe;
use super::wait::wait_isoch;
use crate::constants::{CC_MISSED_SERVICE, CC_SUCCESS};
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::protocol::ISOCH_MAX;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::builders::isoch::isoch;

/// Queue the batch and return the completion code of the batch before it:
/// success, or missed service when the controller reached it late (its
/// packets were dropped, the stream goes on). The first batch returns
/// success at once.
pub fn isoch_out(
    doorbell_base: u64,
    intr_base: u64,
    evt: &mut EventRing,
    pipe: &mut IsochPipe,
    slot: u8,
    lens: &[u16],
    data: &[u8],
) -> XhciResult<u8> {
    let ep = pipe.ep;
    if !batch_fits(lens, data, ep.esit_payload()) {
        return Err(XhciError::ControllerUnsupported);
    }
    if !evt.can_expect() {
        return Err(XhciError::TooManyArmed);
    }
    pipe.ring.room_for(lens.len())?;
    let base = pipe.half * ISOCH_MAX;
    /*
     * SAFETY: the buffer is 2 * ISOCH_MAX bytes and `data` at most ISOCH_MAX;
     * this half's batch completed before the last call returned.
     */
    unsafe {
        let dst = pipe.buf.as_mut_ptr::<u8>().add(base);
        core::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
    }
    let (mut at, mut last) = (pipe.buf.phys() + base as u64, 0u64);
    for (i, &len) in lens.iter().enumerate() {
        let (tbc, tlbpc) = burst_fields(len as u32, ep.max_packet, ep.max_burst, pipe.superspeed);
        let ioc = i + 1 == lens.len();
        let cycle = pipe.ring.cycle() != 0;
        last = pipe.ring.enqueue(isoch(at, len as u32, tbc, tlbpc, ioc, cycle))?;
        at += len as u64;
    }
    ring_doorbell(doorbell_base, slot, ep.dci);
    pipe.half ^= 1;
    let issued = IssuedTransfer { phys: last, slot, dci: ep.dci };
    evt.expect_also(issued);
    let Some(before) = pipe.pending.replace(issued) else { return Ok(CC_SUCCESS) };
    let cc = wait_isoch(intr_base, before, evt)?.completion_code();
    // The batch before ran; this one is still queued behind it.
    pipe.ring.retire_leaving(lens.len());
    match cc {
        CC_SUCCESS | CC_MISSED_SERVICE => Ok(cc),
        cc => Err(XhciError::TransferCompletionFailed(cc)),
    }
}
