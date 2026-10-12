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

//! One batch of isoch IN packets: a TD per packet queued into the free
//! half, each asking for its event, the doorbell rung, then the batch
//! before collected and its bytes packed out. The ring always holds the
//! next batch while the controller fills the last, so a client that keeps
//! asking keeps the endpoint served.

use super::burst::burst_fields;
use super::collect::collect;
use super::inbound::{in_batch_fits, InBatch};
use super::pipe::IsochPipe;
use super::received::{lengths, pack};
use crate::constants::CC_SUCCESS;
use crate::controller::ring_doorbell::ring_doorbell;
use crate::error::{XhciError, XhciResult};
use crate::protocol::{ISOCH_MAX, ISOCH_PACKETS_MAX};
use crate::rings::event::EventRing;
use crate::trb::builders::isoch::isoch;

/// What the batch before held: success, or missed service if any packet of
/// it was not served; its packets' lengths in `lens` and bytes in `out`.
/// Returns the code, the packet count and the bytes packed. The first batch
/// returns none at once.
pub fn isoch_in(
    (doorbell_base, intr_base): (u64, u64),
    evt: &mut EventRing,
    pipe: &mut IsochPipe,
    (slot, n): (u8, usize),
    lens: &mut [u16; ISOCH_PACKETS_MAX],
    out: &mut [u8],
) -> XhciResult<(u8, usize, usize)> {
    let ep = pipe.ep;
    let td_len = ep.esit_payload();
    if !in_batch_fits(n, td_len) {
        return Err(XhciError::ControllerUnsupported);
    }
    let base = pipe.buf.phys() + (pipe.half * ISOCH_MAX) as u64;
    let mut b = InBatch { tds: [0; ISOCH_PACKETS_MAX], n, half: pipe.half, td_len };
    let (tbc, tlbpc) = burst_fields(td_len, ep.max_packet, ep.max_burst, pipe.superspeed);
    for i in 0..n {
        let cycle = pipe.ring.cycle() != 0;
        let at = base + (i as u64) * td_len as u64;
        b.tds[i] = pipe.ring.enqueue(isoch(at, td_len, tbc, tlbpc, true, cycle))?;
    }
    ring_doorbell(doorbell_base, slot, ep.dci);
    pipe.half ^= 1;
    let Some(before) = pipe.inflight.replace(b) else { return Ok((CC_SUCCESS, 0, 0)) };
    let got = collect(intr_base, (slot, ep.dci), &before, evt)?;
    let cc =
        lengths(&got[..before.n], td_len, lens).map_err(XhciError::TransferCompletionFailed)?;
    /*
     * SAFETY: the buffer is 2 * ISOCH_MAX bytes; the batch before was
     * collected to its last TD, so the controller writes that half no more.
     */
    let half = unsafe {
        let p = pipe.buf.as_mut_ptr::<u8>().add(before.half * ISOCH_MAX);
        core::slice::from_raw_parts(p as *const u8, ISOCH_MAX)
    };
    Ok((cc, before.n, pack(half, td_len, &lens[..before.n], out)))
}
