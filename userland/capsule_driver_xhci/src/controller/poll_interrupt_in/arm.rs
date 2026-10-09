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
use crate::error::XhciResult;
use crate::slots::InterruptEndpoint;
use crate::trb::builders::normal::normal;
pub fn arm(
    doorbell_base: u64,
    slot: u8,
    ep: &mut InterruptEndpoint,
    length: u16,
) -> XhciResult<()> {
    let cycle = ep.ring.cycle() != 0;
    let trb = normal(ep.buf.phys(), length as u32, cycle, true, false);
    let issued_phys = ep.ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, slot, ep.dci);
    ep.armed = Some(issued_phys);
    Ok(())
}
