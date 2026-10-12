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

//! The port list in the wire's form, each port's changes cleared once
//! reported, and a slot enabled.

use crate::env::{Clock, Io, Mem};
use crate::facts::io::ROOT_PORTS;
use crate::hc::port::offset;
use crate::hc::wire_portsc::wire_portsc;
use crate::server::context::Ctx;
use crate::server::reply::{data, status, DATA_AT};
use crate::wire::{Request, E_BUSY};

pub fn ports<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
    req: &Request,
    tx: &mut [u8],
) -> usize {
    tx[DATA_AT..DATA_AT + 4].copy_from_slice(&[ROOT_PORTS, 0, 0, 0]);
    for p in 1..=ROOT_PORTS {
        let raw = cx.io.r16(offset(p));
        // Written back, the change bits that were set clear; the rest stay.
        cx.io.w16(offset(p), raw);
        let at = DATA_AT + 4 + 8 * (p as usize - 1);
        tx[at..at + 4].copy_from_slice(&[p, cx.table.owner(p), 0, 0]);
        tx[at + 4..at + 8].copy_from_slice(&wire_portsc(raw).to_le_bytes());
    }
    data(tx, req, 4 + 8 * ROOT_PORTS as usize)
}

pub fn enable<I: Io, C: Clock, M: Mem>(
    cx: &mut Ctx<I, C, M>,
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
