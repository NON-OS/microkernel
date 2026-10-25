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

//! Bulk pipes on an addressed slot: configured once, as driver.xhci0 allows,
//! each on the slot's own ED (reset.rs resets one after a STALL).

use crate::env::{Clock, Mem, Regs};
use crate::sched::ed::Target;
use crate::server::context::Ctx;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_INVAL};
use crate::xfer::bulk::{configure as set_ed, BulkEp};

/// `slot, ep_in, ep_out, bursts, mps_in_le16, mps_out_le16`; the bursts
/// byte is SuperSpeed's and means nothing at full speed.
pub fn configure<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, ep_in, ep_out, _, a, b, c, d] = body else { return status(tx, req, E_INVAL) };
    let shaped = ep_in & 0x80 != 0 && ep_out & 0x80 == 0 && ep_in & 0xF != 0 && ep_out & 0xF != 0;
    let Some(dev) = cx.table.dev(slot).filter(|d| shaped && d.port != 0 && d.bulk.is_none()) else {
        return status(tx, req, E_INVAL);
    };
    let ep = |n: u8, lo: u8, hi: u8, index: usize| BulkEp {
        t: Target { addr: slot, ep: n & 0xF, low_speed: dev.low_speed },
        mps: u16::from_le_bytes([lo, hi]) & 0x7FF,
        index,
        at: 0,
    };
    let first = 2 * (slot as usize - 1);
    let (i, o) = (ep(ep_in, a, b, first), ep(ep_out, c, d, first + 1));
    set_ed(&cx.mem, &i);
    set_ed(&cx.mem, &o);
    dev.bulk = Some((i, o));
    dev.claimed = true;
    bytes(tx, req, &[2 * (ep_in & 0xF) + 1, 2 * (ep_out & 0xF), 0, 0])
}
