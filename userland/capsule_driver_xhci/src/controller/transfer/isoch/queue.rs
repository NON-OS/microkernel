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
    pipe.ring.room_for(lens.len())?;
    // The batch before holds a place, given back before this one takes it.
    if pipe.pending.is_none() && !evt.can_expect() {
        return Err(XhciError::TooManyArmed);
    }
    let last = queue_tds(pipe, lens, data)?;
    ring_doorbell(doorbell_base, slot, ep.dci);
    let issued = IssuedTransfer { phys: last, slot, dci: ep.dci };
    let cc = match pipe.pending.replace(issued) {
        None => CC_SUCCESS,
        Some(before) => match wait_isoch(intr_base, before, evt)?.completion_code() {
            cc @ (CC_SUCCESS | CC_MISSED_SERVICE) => cc,
            cc => return Err(XhciError::TransferCompletionFailed(cc)),
        },
    };
    // The batch before ran whole: only this one may still wait on the ring.
    pipe.ring.retire_but(lens.len());
    evt.expect(issued);
    Ok(cc)
}
