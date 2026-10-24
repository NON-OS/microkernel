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

//! Any control transfer a class driver asks for on EP0: its OUT data taken
//! from the request, the data stage given back after a u16 of its length.

use super::ep0::{ep0, give};
use crate::env::{Clock, Mem, Regs};
use crate::sched::layout::{CTRL_BUF, CTRL_MAX};
use crate::server::context::Ctx;
use crate::server::reply::status;
use crate::wire::{Request, E_INVAL};

/// `slot, 0, bmRequestType, bRequest, wValue, wIndex, wLength`, then an OUT
/// request's data.
pub fn control<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let Some((head, out)) = body.split_first_chunk::<10>() else {
        return status(tx, req, E_INVAL);
    };
    let mut setup = [0u8; 8];
    setup.copy_from_slice(&head[2..]);
    let len = u16::from_le_bytes([setup[6], setup[7]]) as usize;
    let input = setup[0] & 0x80 != 0;
    if len > CTRL_MAX || (!input && out.len() != len) || (input && !out.is_empty()) {
        return status(tx, req, E_INVAL);
    }
    cx.mem.write_bytes(CTRL_BUF, out);
    match ep0(cx, head[0], setup) {
        // OUT data comes back as driver.xhci0 gives it: the bytes sent.
        Ok(n) => give(cx, req, tx, n, 2),
        Err(e) => status(tx, req, e),
    }
}
