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
//! One bulk transfer through the slot's DMA page: a single Normal TRB, the
//! page never crossing the 64 KiB boundary a TRB may not span.
use super::pipes::BulkPipes;
use super::wait::wait_bulk;
use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS};
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::protocol::BULK_MAX;
use crate::rings::event::EventRing;
use crate::trb::builders::normal::normal;
/// Move `len` bytes, 1 to `BULK_MAX`, IN to or OUT of `pipes.buf`. Returns
/// the bytes moved; a STALL or any other failure comes back as its
/// completion code for the class driver to recover from.
pub fn bulk_transfer(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    pipes: &mut BulkPipes,
    slot: u8,
    dir_in: bool,
    len: u32,
) -> XhciResult<u32> {
    if len == 0 || len as usize > BULK_MAX {
        return Err(XhciError::ControllerUnsupported);
    }
    let buf_phys = pipes.buf.phys();
    let (ring, dci) = if dir_in {
        (&mut pipes.in_ring, pipes.dci_in)
    } else {
        (&mut pipes.out_ring, pipes.dci_out)
    };
    let issued = ring.enqueue(normal(buf_phys, len, ring.cycle() != 0, true, false))?;
    ring_doorbell(doorbell_base, slot, dci);
    let event = wait_bulk(intr_base, issued, evt_ring)?;
    match event.completion_code() {
        CC_SUCCESS | CC_SHORT_PACKET => Ok(len - (event.d2 & 0x00FF_FFFF).min(len)),
        cc => Err(XhciError::TransferCompletionFailed(cc)),
    }
}
