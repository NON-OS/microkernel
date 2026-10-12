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

//! The port list in the wire's form, each port's connect change cleared
//! once reported, and a slot enabled. A port register's low bits written
//! are commands, so only the change bit is written back.

use crate::env::{Clock, Mem, Regs};
use crate::facts::port::PORT_CSC;
use crate::hc::port::offset;
use crate::hc::wire_portsc::wire_portsc;
use crate::server::context::Ctx;
use crate::server::reply::{data, status, DATA_AT};
use crate::wire::{Request, E_BUSY};

pub fn ports<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    tx: &mut [u8],
) -> usize {
    let ports = cx.table.ports;
    tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[ports, 0, 0, 0]);
    for p in 1..=ports {
        let raw = cx.io.r32(offset(p));
        if raw & PORT_CSC != 0 {
            cx.io.w32(offset(p), PORT_CSC);
        }
        let at = DATA_AT + 4 + 8 * (p as usize - 1);
        tx[at..at + 4].copy_from_slice(&[p, cx.table.owner(p), 0, 0]);
        tx[at + 4..at + 8].copy_from_slice(&wire_portsc(raw).to_le_bytes());
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
