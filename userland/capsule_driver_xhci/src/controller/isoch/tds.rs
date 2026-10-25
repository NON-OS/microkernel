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

//! A batch's packets put in the free half of the pipe's memory and queued,
//! a TD each.

use super::burst::burst_fields;
use super::pipe::IsochPipe;
use crate::error::{XhciError, XhciResult};
use crate::protocol::ISOCH_MAX;
use crate::trb::builders::isoch::isoch;

/// Copy `data` into the half no TD in flight reads, its batch completed
/// before the last call returned, and queue one TD per packet, IOC on the
/// last. Returns that last TD's address. The ring has room for them all.
pub(super) fn queue_tds(pipe: &mut IsochPipe, lens: &[u16], data: &[u8]) -> XhciResult<u64> {
    let ep = pipe.ep;
    let base = pipe.half * ISOCH_MAX;
    if !pipe.buf.copy_from_at(base, data) {
        return Err(XhciError::ControllerUnsupported);
    }
    let (mut at, mut last) = (pipe.buf.phys() + base as u64, 0u64);
    for (i, &len) in lens.iter().enumerate() {
        let (tbc, tlbpc) = burst_fields(len as u32, ep.max_packet, ep.max_burst, pipe.superspeed);
        let ioc = i + 1 == lens.len();
        let cycle = pipe.ring.cycle() != 0;
        last = pipe.ring.enqueue(isoch(at, len as u32, tbc, tlbpc, ioc, cycle))?;
        at += len as u64;
    }
    Ok(last)
}
