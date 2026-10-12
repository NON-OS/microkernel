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

//! The port list in the wire's form, each port decided first: a port
//! newly connected (or whose connection changed) is routed (hc/route.rs),
//! so only high-speed devices show here and the rest show on the companion
//! host. The connect change is cleared once reported.

use crate::env::{Clock, Mem, Regs};
use crate::facts::port::{PORT_CCS, PORT_CSC, PORT_WC};
use crate::hc::port::offset;
use crate::hc::route::{route, Route};
use crate::hc::wire_portsc::wire_portsc;
use crate::server::context::Ctx;
use crate::server::reply::{data, status, DATA_AT};
use crate::wire::{Request, E_BUSY};

pub fn ports<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    tx: &mut [u8],
) -> usize {
    let (ports, op) = (cx.table.ports, cx.table.op);
    tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[ports, 0, 0, 0]);
    for p in 1..=ports {
        let (off, i) = (offset(op, p), p as usize - 1);
        let raw = cx.io.r32(off);
        let changed = raw & PORT_CSC != 0;
        if changed || raw & PORT_CCS == 0 {
            cx.table.high[i] = false;
        }
        if raw & PORT_CCS != 0 && !cx.table.high[i] {
            cx.table.high[i] = route(&mut cx.io, &cx.clock, op, p) == Route::High;
        }
        let raw = cx.io.r32(off);
        if raw & PORT_CSC != 0 {
            cx.io.w32(off, raw & !PORT_WC | PORT_CSC);
        }
        let at = DATA_AT + 4 + 8 * i;
        tx[at..at + 4].copy_from_slice(&[p, cx.table.owner(p), 0, 0]);
        let csc = if changed { PORT_CSC } else { 0 };
        let shown = wire_portsc(raw | csc, cx.table.high[i]);
        tx[at + 4..at + 8].copy_from_slice(&shown.to_le_bytes());
    }
    data(tx, req, 4 + 8 * ports as usize)
}

pub fn enable<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    tx: &mut [u8],
) -> usize {
    match cx.table.enable() {
        Some(slot) => {
            tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[slot, 0, 0, 0]);
            data(tx, req, 4)
        }
        None => status(tx, req, E_BUSY),
    }
}
