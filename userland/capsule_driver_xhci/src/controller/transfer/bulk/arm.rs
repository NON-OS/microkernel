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
//! Arming the polled bulk IN: one Normal TRB into the poll page.
use super::pipes::BulkPipes;
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::builders::normal::normal;

/// Arm `len` bytes on the IN ring with a place kept for the event, which
/// another wait may read first; refused before anything is queued when
/// every place is taken. Returns the TRB's address and `len`.
pub(super) fn arm(
    doorbell_base: u64,
    evt_ring: &mut EventRing,
    pipes: &mut BulkPipes,
    slot: u8,
    len: u32,
) -> XhciResult<(u64, u32)> {
    if !evt_ring.can_expect() {
        return Err(XhciError::TooManyArmed);
    }
    let trb = normal(pipes.poll_buf.phys(), len, pipes.in_ring.cycle() != 0, true, false);
    let phys = pipes.in_ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, slot, pipes.dci_in);
    evt_ring.expect(IssuedTransfer { phys, slot, dci: pipes.dci_in });
    pipes.armed = Some((phys, len));
    Ok((phys, len))
}
