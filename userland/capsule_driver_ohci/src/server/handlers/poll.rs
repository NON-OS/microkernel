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

//! The polled bulk IN, `slot, 0, len_le16`: the first poll arms a TD of
//! `len` bytes, and each poll looks at it once. Data replies as a bulk IN
//! does; nothing yet is E_AGAIN at once; a STALL is E_PIPE. The TD is armed
//! again by the poll after the one that took its data. Its buffer has one
//! place, so while one slot's is armed another slot's poll is E_BUSY.

use crate::env::{Clock, Mem, Regs};
use crate::sched::layout::{BULK_MAX, POLL_BUF};
use crate::server::context::Ctx;
use crate::server::reply::{data, status, DATA_AT};
use crate::wire::{Request, E_AGAIN, E_BUSY, E_INVAL, E_IO, E_PIPE};
use crate::xfer::poll::{arm, look, Polled};

pub fn poll<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, _, lo, hi] = body else { return status(tx, req, E_INVAL) };
    let len = u16::from_le_bytes([lo, hi]) as usize;
    let mut armed =
        cx.table.devs.iter().enumerate().filter(|(_, d)| d.is_some_and(|d| d.poll.is_some()));
    if armed.any(|(i, _)| i + 1 != slot as usize) {
        return status(tx, req, E_BUSY);
    }
    let Some(dev) = cx.table.dev(slot).filter(|_| (1..=BULK_MAX).contains(&len)) else {
        return status(tx, req, E_INVAL);
    };
    let Some((ep, _)) = dev.bulk.as_mut() else { return status(tx, req, E_INVAL) };
    let n = match dev.poll {
        Some(n) => n,
        None => match arm(&cx.mem, &mut cx.io, ep, len) {
            Ok(()) => *dev.poll.insert(len),
            Err(_) => return status(tx, req, E_INVAL),
        },
    };
    let polled = look(&cx.mem, &mut cx.io, ep, n);
    if polled != Polled::Pending {
        dev.poll = None;
    }
    match polled {
        Polled::Pending => status(tx, req, E_AGAIN),
        Polled::Done(got) => {
            // The TD was armed for an earlier poll's length; give no more
            // than this one asked for.
            let n = got.min(len);
            tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[n as u8, (n >> 8) as u8, 0, 0]);
            cx.mem.read_bytes(POLL_BUF, &mut tx[DATA_AT + 4..DATA_AT + 4 + n]);
            data(tx, req, 4 + n)
        }
        Polled::Stalled => status(tx, req, E_PIPE),
        Polled::Error(_) => status(tx, req, E_IO),
    }
}
