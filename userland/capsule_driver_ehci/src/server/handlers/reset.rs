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

//! A bulk pipe reset after a STALL: the QH taken back with DATA0 next,
//! which also drops a polled qTD armed on it.

use crate::env::{Clock, Mem, Regs};
use crate::server::context::Ctx;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_INVAL};
use crate::xfer::bulk_run::reset as reset_ed;

/// `slot, dir_in`: the next packet carries DATA0 again, matching the
/// device once the class driver clears the halt.
pub fn reset<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, dir_in @ (0 | 1)] = body else { return status(tx, req, E_INVAL) };
    let Some(dev) = cx.table.dev(slot) else { return status(tx, req, E_INVAL) };
    let Some((i, o)) = dev.bulk.as_mut() else { return status(tx, req, E_INVAL) };
    let ep = if dir_in == 1 {
        dev.poll = None;
        *i
    } else {
        *o
    };
    let op = cx.table.op;
    reset_ed((&cx.mem, &mut cx.io, &cx.clock), op, &ep);
    bytes(tx, req, &[0; 4])
}
