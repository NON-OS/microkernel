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
//! One bulk transfer on a stream: the Normal TRB on the stream's own ring,
//! and the doorbell naming the stream beside the endpoint.

use super::pipes::BulkPipes;
use super::transfer::moved;
use super::wait::wait_bulk;
use crate::controller::ring_doorbell::ring_doorbell_stream;
use crate::error::{XhciError, XhciResult};
use crate::protocol::BULK_MAX;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::builders::normal::normal;

/// As `bulk_transfer`, on stream `stream` (1 up) of a streamed endpoint.
pub fn stream_transfer(
    (doorbell_base, intr_base): (u64, u64),
    evt_ring: &mut EventRing,
    pipes: &mut BulkPipes,
    (slot, dir_in, stream): (u8, bool, u16),
    len: u32,
) -> XhciResult<u32> {
    if len == 0 || len as usize > BULK_MAX {
        return Err(XhciError::ControllerUnsupported);
    }
    let buf_phys = pipes.buf.phys();
    let (streams, dci) = if dir_in {
        (pipes.in_streams.as_mut(), pipes.dci_in)
    } else {
        (pipes.out_streams.as_mut(), pipes.dci_out)
    };
    let ring = streams.and_then(|s| s.ring_mut(stream)).ok_or(XhciError::ControllerUnsupported)?;
    let issued = ring.enqueue(normal(buf_phys, len, ring.cycle() != 0, true, false))?;
    ring_doorbell_stream(doorbell_base, slot, dci, stream);
    let event = wait_bulk(intr_base, IssuedTransfer { phys: issued, slot, dci }, evt_ring)?;
    moved(ring, event, len)
}
