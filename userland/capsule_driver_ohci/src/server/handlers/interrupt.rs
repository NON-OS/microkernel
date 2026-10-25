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

//! Interrupt IN endpoints: one armed on its own queue when a class driver
//! gives it a "transfer ring", then polled. The reply carries the report's
//! length as a u16 and the report; E_AGAIN while the device has none.

use crate::env::{Clock, Mem, Regs};
use crate::sched::ed::Target;
use crate::server::context::Ctx;
use crate::server::reply::{data, status, DATA_AT};
use crate::wire::{Request, E_AGAIN, E_BUSY, E_INVAL, E_IO, E_PIPE};
use crate::xfer::interrupt::{arm, IntrEp};
use crate::xfer::interrupt_poll::{poll, Poll};

/// `slot, ep_address, mps_le16, interval, 0`; reply `dci, 0, 0, 0`.
pub fn alloc<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, ep_addr, lo, hi, _, _] = body else { return status(tx, req, E_INVAL) };
    let Some(dev) = cx.table.dev(slot).filter(|d| d.port != 0 && ep_addr & 0x80 != 0) else {
        return status(tx, req, E_INVAL);
    };
    dev.claimed = true;
    let t = Target { addr: slot, ep: ep_addr & 0x0F, low_speed: dev.low_speed };
    let Some(index) = cx.table.intr.iter().position(|e| e.is_none()) else {
        return status(tx, req, E_BUSY);
    };
    let mps = u16::from_le_bytes([lo, hi]);
    let ep = IntrEp { t, index, mps, len: mps as usize, at: 0 };
    arm(&cx.mem, &mut cx.io, &ep);
    cx.table.intr[index] = Some((slot, ep));
    tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[2 * (ep_addr & 0x0F) + 1, 0, 0, 0]);
    data(tx, req, 4)
}

/// `slot, dci, len_le16`.
pub fn read<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, dci, _, _] = body else { return status(tx, req, E_INVAL) };
    let found =
        cx.table.intr.iter_mut().flatten().find(|(s, e)| *s == slot && 2 * e.t.ep + 1 == dci);
    let Some((_, ep)) = found else { return status(tx, req, E_INVAL) };
    let mut report = [0u8; 64];
    match poll(&cx.mem, &mut cx.io, ep, &mut report) {
        Poll::Pending => status(tx, req, E_AGAIN),
        Poll::Report(n) => {
            tx[DATA_AT..DATA_AT + 2].copy_from_slice(&(n as u16).to_le_bytes());
            tx[DATA_AT + 2..DATA_AT + 2 + n].copy_from_slice(&report[..n]);
            data(tx, req, 2 + n)
        }
        Poll::Stalled => status(tx, req, E_PIPE),
        Poll::Error(_) => status(tx, req, E_IO),
    }
}
