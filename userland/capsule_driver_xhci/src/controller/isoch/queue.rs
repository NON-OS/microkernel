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
use super::pipe::IsochPipe;
use super::tds::queue_tds;
use super::wait::wait_isoch;
use crate::constants::{CC_MISSED_SERVICE, CC_SUCCESS};
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::rings::event::{EventRing, IssuedTransfer};

/// Queue the batch and return the completion code of the batch before it:
/// success, or missed service when the controller reached it late (its
/// packets were dropped, the stream goes on). The first batch returns
/// success at once. Nothing is queued unless the whole batch fits the ring
/// and its last TD's event has a place to be kept in.
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
    let last = queue_tds(pipe, lens, data)?;
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
