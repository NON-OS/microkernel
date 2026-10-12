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

//! A slot given back, its interrupt queues and polled chain emptied and its
//! port disabled; and a device addressed on a free root port.

use crate::devices::address::{address as run_address, AddrError};
use crate::env::{Clock, Io, Mem};
use crate::hc::port::disable as port_off;
use crate::hc::wire_portsc::speed_id;
use crate::sched::layout::intr_qh;
use crate::sched::qh::stop;
use crate::server::context::Ctx;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_BUSY, E_INVAL, E_IO};
use crate::xfer::poll::disarm;

pub fn disable<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let Some(dev) = body.first().and_then(|&s| cx.table.dev(s)).copied() else {
        return status(tx, req, E_INVAL);
    };
    let slot = body[0];
    for (i, e) in cx.table.intr.iter_mut().enumerate() {
        if e.is_some_and(|(s, _)| s == slot) {
            stop(&cx.mem, intr_qh(i));
            *e = None;
        }
    }
    if dev.poll.is_some() {
        disarm(&cx.mem, dev.port as usize - 1);
    }
    // A device below a hub leaves the root port, and the hub on it, alone.
    if dev.port != 0 && dev.route == 0 {
        port_off(&mut cx.io, dev.port);
    }
    cx.table.devs[slot as usize - 1] = None;
    status(tx, req, 0)
}

pub fn address<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, port] = body else { return status(tx, req, E_INVAL) };
    match run_address(&mut cx.io, &cx.clock, &cx.mem, &mut cx.table, (slot, port)) {
        Ok((low, mps0)) => {
            let [m0, m1] = mps0.to_le_bytes();
            bytes(tx, req, &[slot, port, speed_id(low), 0, m0, m1, 0, 0])
        }
        Err(AddrError::Busy) => status(tx, req, E_BUSY),
        Err(AddrError::BadRequest) => status(tx, req, E_INVAL),
        Err(_) => status(tx, req, E_IO),
    }
}
