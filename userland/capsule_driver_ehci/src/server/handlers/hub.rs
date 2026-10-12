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
//! The hub ops. OP_CONFIGURE_HUB marks an addressed slot a hub: the
//! controller needs nothing more, since a QH names its TT itself.
//! OP_ADDRESS_ROUTED addresses a device below a hub, which reset its own
//! port; the root port is left alone (resetting it resets the hub).

use super::routed::{below, Below};
use crate::devices::address::AddrError;
use crate::devices::assign::assign;
use crate::env::{Clock, Mem, Regs};
use crate::server::context::Ctx;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_INVAL, E_IO};

/// `slot, ports, think_time, mtt`: status only.
pub fn configure<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let &[slot, ports, 0..=3, 0 | 1] = body else { return status(tx, req, E_INVAL) };
    match cx.table.dev(slot).filter(|d| d.port != 0 && ports != 0) {
        Some(dev) => {
            dev.hub = true;
            status(tx, req, 0)
        }
        None => status(tx, req, E_INVAL),
    }
}

/// `slot, root_port, speed, route_le32, tt_slot, tt_port`; answered as
/// OP_ADDRESS_DEVICE is.
pub fn address<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let Some(Below { slot, port, route, link }) = below(&cx.table, body) else {
        return status(tx, req, E_INVAL);
    };
    match assign((&cx.mem, &mut cx.io, &cx.clock), &mut cx.table, (slot, port), link) {
        Ok(mps0) => {
            if let Some(d) = cx.table.dev(slot) {
                d.route = route;
            }
            let [m0, m1] = mps0.to_le_bytes();
            bytes(tx, req, &[slot, port, link.speed, 0, m0, m1, 0, 0])
        }
        Err(AddrError::BadRequest) => status(tx, req, E_INVAL),
        Err(_) => status(tx, req, E_IO),
    }
}
