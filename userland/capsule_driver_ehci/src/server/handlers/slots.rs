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

//! A slot given back, its interrupt QHs parked, its bulk QHs taken back
//! at DATA0 and its port disabled; and a device addressed on a root port.

use crate::devices::address::{address as run_address, AddrError};
use crate::env::{Clock, Mem, Regs};
use crate::hc::port::{disable as port_off, offset};
use crate::hc::wire_portsc::SPEED_HIGH;
use crate::server::context::Ctx;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_BUSY, E_INVAL, E_IO};
use crate::xfer::bulk_run::reset as retire;
use crate::xfer::interrupt::stop;

pub fn disable<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let Some(dev) = body.first().and_then(|&s| cx.table.dev(s)).copied() else {
        return status(tx, req, E_INVAL);
    };
    let (slot, op) = (body[0], cx.table.op);
    for (i, e) in cx.table.intr.iter_mut().enumerate() {
        if e.is_some_and(|(s, _)| s == slot) {
            stop((&cx.mem, &mut cx.io, &cx.clock), op, i);
            *e = None;
        }
    }
    // Taken back with any polled qTD on them, DATA0 for the next slot.
    if let Some((i, o)) = dev.bulk {
        retire((&cx.mem, &mut cx.io, &cx.clock), op, &i);
        retire((&cx.mem, &mut cx.io, &cx.clock), op, &o);
    }
    // A device below a hub leaves the root port, and the hub on it, alone.
    if dev.port != 0 && dev.route == 0 {
        port_off(&mut cx.io, offset(op, dev.port));
    }
    cx.table.devs[slot as usize - 1] = None;
    status(tx, req, 0)
}

pub fn address<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, port] = body else { return status(tx, req, E_INVAL) };
    match run_address((&cx.mem, &mut cx.io, &cx.clock), &mut cx.table, (slot, port)) {
        Ok(mps0) => {
            let [m0, m1] = mps0.to_le_bytes();
            bytes(tx, req, &[slot, port, SPEED_HIGH, 0, m0, m1, 0, 0])
        }
        Err(AddrError::Busy) => status(tx, req, E_BUSY),
        Err(AddrError::BadRequest) => status(tx, req, E_INVAL),
        Err(_) => status(tx, req, E_IO),
    }
}
