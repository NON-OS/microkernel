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

//! The device descriptor, 18 raw bytes, and the configuration descriptor,
//! after a u16 of the bytes read and two zeros, as driver.xhci0 gives them.

use super::ep0::{ep0, give};
use crate::env::{Clock, Mem, Regs};
use crate::sched::layout::CTRL_BUF;
use crate::server::context::Ctx;
use crate::server::reply::{data, status, DATA_AT};
use crate::wire::{Request, E_INVAL, E_IO};

/// `slot`.
pub fn device<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot] = body else { return status(tx, req, E_INVAL) };
    match ep0(cx, slot, [0x80, 6, 0, 1, 0, 0, 18, 0]) {
        Ok(18) => {
            cx.mem.read_bytes(CTRL_BUF, &mut tx[DATA_AT..DATA_AT + 18]);
            data(tx, req, 18)
        }
        Ok(_) => status(tx, req, E_IO),
        Err(e) => status(tx, req, e),
    }
}

/// `slot, 0, len_le16`.
pub fn config<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, _, lo, hi] = body else { return status(tx, req, E_INVAL) };
    match ep0(cx, slot, [0x80, 6, 0, 2, 0, 0, lo, hi]) {
        Ok(n) => give(cx, req, tx, n, 4),
        Err(e) => status(tx, req, e),
    }
}
