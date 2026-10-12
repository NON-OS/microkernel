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

//! A bulk transfer: `slot, pair, len_le16`, then an OUT's data (pair 0
//! the first bulk pair, 1 the second, bulk2.rs). The reply is
//! `moved_le16, 0, 0`, and an IN's bytes after it; a STALL is E_PIPE, so
//! the class driver resets the pipe. An IN while the polled TD is armed
//! is refused: both would take the endpoint's toggle.

use super::bulk2::pair;
use crate::env::{Clock, Mem, Regs};
use crate::sched::layout::{BULK_BUF, BULK_MAX};
use crate::server::context::Ctx;
use crate::server::reply::{data, status, DATA_AT};
use crate::wire::{Request, E_BUSY, E_INVAL, E_IO, E_PIPE};
use crate::xfer::bulk_run::bulk;
use crate::xfer::failed::CtrlError;

pub fn transfer<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
    input: bool,
) -> usize {
    let Some((&[slot, which, lo, hi], out)) = body.split_first_chunk::<4>() else {
        return status(tx, req, E_INVAL);
    };
    let len = u16::from_le_bytes([lo, hi]) as usize;
    let fits = (1..=BULK_MAX).contains(&len) && out.len() == if input { 0 } else { len };
    let op = cx.table.op;
    let Some(dev) = cx.table.dev(slot).filter(|_| fits) else { return status(tx, req, E_INVAL) };
    if input && which == 0 && dev.poll.is_some() {
        return status(tx, req, E_BUSY);
    }
    let Some((i, o)) = pair(dev, which) else { return status(tx, req, E_INVAL) };
    let ep = if input { i } else { o };
    cx.mem.write_bytes(BULK_BUF, out);
    match bulk((&cx.mem, &mut cx.io, &cx.clock), op, ep, input, len) {
        Ok(n) => {
            tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[n as u8, (n >> 8) as u8, 0, 0]);
            let n = if input { n } else { 0 };
            cx.mem.read_bytes(BULK_BUF, &mut tx[DATA_AT + 4..DATA_AT + 4 + n]);
            data(tx, req, 4 + n)
        }
        Err(CtrlError::Stalled) => status(tx, req, E_PIPE),
        Err(CtrlError::TooLong) => status(tx, req, E_INVAL),
        Err(_) => status(tx, req, E_IO),
    }
}
