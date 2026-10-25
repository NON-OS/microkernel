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
//! The second bulk pair (OP_CONFIGURE_BULK2): UAS's data pipes beside its
//! command and status pipes, on endpoints the first pair does not hold.
//! Its QHs are one of SECOND_PAIRS places in the async ring, held while
//! the slot holds the pair. Bulk ops name it as pair 1 in their second
//! byte; RESET_BULK in bit 1.

use crate::devices::table::{Dev, Table};
use crate::env::{Clock, Mem, Regs};
use crate::sched::layout::{pair2_ring, SECOND_PAIRS};
use crate::server::context::Ctx;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_BUSY, E_INVAL};
use crate::xfer::bulk::{configure as set_qh, BulkEp};

/// As OP_CONFIGURE_BULK: `slot, ep_in, ep_out, bursts, mps_in, mps_out`.
pub fn configure<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, ep_in, ep_out, _, a, b, c, d] = body else { return status(tx, req, E_INVAL) };
    let shaped = ep_in & 0x80 != 0 && ep_out & 0x80 == 0 && ep_in & 0xF != 0 && ep_out & 0xF != 0;
    let place = (0..SECOND_PAIRS).find(|&p| !held(&cx.table, p));
    let clear =
        |d: &&mut Dev| d.bulk.is_none_or(|(i, o)| i.ep != ep_in & 0xF && o.ep != ep_out & 0xF);
    let fits = |d: &&mut Dev| shaped && d.port != 0 && d.bulk2.is_none() && clear(d);
    let Some(dev) = cx.table.dev(slot).filter(fits) else { return status(tx, req, E_INVAL) };
    let Some(place) = place else { return status(tx, req, E_BUSY) };
    let link = dev.link;
    let ep = |n: u8, lo: u8, hi: u8, input: bool| BulkEp {
        addr: slot,
        ep: n & 0xF,
        mps: u16::from_le_bytes([lo, hi]) & 0x7FF,
        ring: pair2_ring(place, input),
        link,
    };
    let (i, o) = (ep(ep_in, a, b, true), ep(ep_out, c, d, false));
    (dev.bulk2, dev.claimed) = (Some((i, o)), true);
    let op = cx.table.op;
    set_qh((&cx.mem, &mut cx.io, &cx.clock), op, &i);
    set_qh((&cx.mem, &mut cx.io, &cx.clock), op, &o);
    bytes(tx, req, &[2 * (ep_in & 0xF) + 1, 2 * (ep_out & 0xF), 0, 0])
}

/// Whether a slot holds second-pair place `p`.
fn held(t: &Table, p: usize) -> bool {
    let ring = pair2_ring(p, true);
    t.devs.iter().flatten().any(|d| d.bulk2.is_some_and(|(i, _)| i.ring == ring))
}

/// The pair a bulk op's second byte names: 0 the first, 1 the second.
pub fn pair(dev: &mut Dev, byte: u8) -> Option<&mut (BulkEp, BulkEp)> {
    match byte {
        0 => dev.bulk.as_mut(),
        1 => dev.bulk2.as_mut(),
        _ => None,
    }
}
