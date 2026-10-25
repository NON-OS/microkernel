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
//! The polled bulk IN a network class driver reads with. One TRB stays armed
//! on the IN ring into the poll page, with a place kept for its event, and
//! each look either takes its completion or answers at once, so an idle link
//! never holds the controller as a waited bulk IN (a USB stick's) does for
//! up to five seconds.
use super::arm::arm;
use super::look::look;
use super::pipes::BulkPipes;
use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS};
use crate::error::{XhciError, XhciResult};
use crate::protocol::BULK_MAX;
use crate::rings::event::{EventRing, IssuedTransfer};

#[derive(Debug, PartialEq, Eq)]
pub enum BulkPoll {
    /// The bytes that came in, now at the start of `pipes.poll_buf`.
    Complete(u32),
    Pending,
}

/// Look once at the polled IN, arming `len` bytes (1 to `BULK_MAX`) first
/// when nothing is armed. A completion ending in an error comes back as its
/// code; the TRB is spent either way, so the next look arms a fresh one.
pub fn poll_bulk_in(
    doorbell_base: u64,
    intr_base: u64,
    evt_ring: &mut EventRing,
    pipes: &mut BulkPipes,
    slot: u8,
    len: u32,
) -> XhciResult<BulkPoll> {
    // A streamed IN is read on its streams, never polled.
    if len == 0 || len as usize > BULK_MAX || pipes.in_streams.is_some() {
        return Err(XhciError::ControllerUnsupported);
    }
    let (phys, asked) = match pipes.armed {
        Some(armed) => armed,
        None => arm(doorbell_base, evt_ring, pipes, slot, len)?,
    };
    let issued = IssuedTransfer { phys, slot, dci: pipes.dci_in };
    let Some(event) = look(intr_base, evt_ring, issued) else {
        return Ok(BulkPoll::Pending);
    };
    // `look` gave the event's place back. The controller ran every TRB on
    // the ring up to this one, which is spent whatever its code.
    pipes.armed = None;
    pipes.in_ring.retire();
    match event.completion_code() {
        // The residual clamped to the request, as the waited transfer reads it.
        CC_SUCCESS | CC_SHORT_PACKET => {
            Ok(BulkPoll::Complete(asked - (event.d2 & 0xFF_FFFF).min(asked)))
        }
        cc => Err(XhciError::TransferCompletionFailed(cc)),
    }
}
