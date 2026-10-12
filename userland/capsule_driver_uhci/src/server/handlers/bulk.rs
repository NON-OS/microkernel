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
//! Any control transfer a class driver asks for on EP0: its OUT data taken
//! Bulk pipes on an addressed slot: configured once, as driver.xhci0 allows,
//! and reset after a STALL. UHCI keeps no endpoint state of its own, so
//! both are the slot's toggles; a reset IN pipe also drops the polled chain.

use crate::env::{Clock, Io, Mem};
use crate::sched::td::Target;
use crate::server::context::Ctx;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_INVAL};
use crate::xfer::bulk::BulkEp;
use crate::xfer::poll::disarm;

/// `slot, ep_in, ep_out, bursts, mps_in_le16, mps_out_le16`; the bursts
/// byte is SuperSpeed's and means nothing at full speed.
pub fn configure<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, ep_in, ep_out, _, a, b, c, d] = body else { return status(tx, req, E_INVAL) };
    let shaped = ep_in & 0x80 != 0 && ep_out & 0x80 == 0 && ep_in & 0xF != 0 && ep_out & 0xF != 0;
    let Some(dev) = cx.table.dev(slot).filter(|d| shaped && d.port != 0 && d.bulk.is_none()) else {
        return status(tx, req, E_INVAL);
    };
    let ep = |n: u8, lo: u8, hi: u8| BulkEp {
        t: Target { addr: slot, ep: n & 0xF, low_speed: dev.low_speed },
        mps: u16::from_le_bytes([lo, hi]) & 0x7FF,
        toggle: false,
    };
    dev.bulk = Some((ep(ep_in, a, b), ep(ep_out, c, d)));
    dev.claimed = true;
    bytes(tx, req, &[2 * (ep_in & 0xF) + 1, 2 * (ep_out & 0xF), 0, 0])
}

/// `slot, dir_in`: the next packet carries DATA0 again, matching the
/// device once the class driver clears the halt.
pub fn reset<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, dir_in @ (0 | 1)] = body else { return status(tx, req, E_INVAL) };
    let Some(dev) = cx.table.dev(slot) else { return status(tx, req, E_INVAL) };
    let Some((i, o)) = dev.bulk.as_mut() else { return status(tx, req, E_INVAL) };
    if dir_in == 1 {
        i.toggle = false;
        if dev.poll.take().is_some() {
            disarm(&cx.mem);
        }
    } else {
        o.toggle = false;
    }
    bytes(tx, req, &[0; 4])
}
