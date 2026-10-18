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

use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::slots::InterruptEndpoint;
use crate::trb::builders::normal::normal;
/// Arm one report on `ep`, as long as the endpoint's payload, with a place
/// kept for its event; refused before anything is queued when every place
/// is taken. A TRB shorter than what the device sends would end in babble
/// and a halted endpoint, whatever the client asked for.
pub fn arm(
    doorbell_base: u64,
    evt_ring: &mut EventRing,
    slot: u8,
    ep: &mut InterruptEndpoint,
) -> XhciResult<()> {
    if !evt_ring.can_expect() {
        return Err(XhciError::TooManyArmed);
    }
    let cycle = ep.ring.cycle() != 0;
    let trb = normal(ep.buf.phys(), ep.len as u32, cycle, true, false);
    let phys = ep.ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, slot, ep.dci);
    ep.armed = Some(phys);
    evt_ring.expect(IssuedTransfer { phys, slot, dci: ep.dci });
    Ok(())
}
