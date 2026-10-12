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
//! One request to the host: the port list from every controller, slots
//! given out here, an address request to the controller whose port it
//! names (the port renumbered both ways), and anything else to the
//! controller its slot is bound to.

use super::front::{Bind, Front};
use super::hc::Hc;
use super::ports::ports;
use crate::server::reply::{bytes, status, DATA_AT};
use crate::wire::ops::*;
use crate::wire::{Request, E_BUSY, E_INVAL, HDR_LEN};

pub fn serve<H: Hc>(f: &mut Front<H>, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let slot = body.first().copied().unwrap_or(0);
    let at = (slot as usize).wrapping_sub(1);
    let bind = f.bound.get(at).copied();
    match (req.op, bind) {
        (OP_HEALTHCHECK, _) => status(tx, req, 0),
        (OP_PORT_STATUS, _) => ports(f, req, tx),
        (OP_ENABLE_SLOT, _) => match f.bound.iter().position(|b| *b == Bind::Free) {
            Some(i) => {
                f.bound[i] = Bind::Enabled;
                bytes(tx, req, &[i as u8 + 1, 0, 0, 0])
            }
            None => status(tx, req, E_BUSY),
        },
        (OP_DISABLE_SLOT, Some(Bind::Enabled)) => {
            f.bound[at] = Bind::Free;
            status(tx, req, 0)
        }
        (OP_ADDRESS_DEVICE | OP_ADDRESS_ROUTED, Some(Bind::Enabled)) if body.len() >= 2 => {
            let Some((k, local)) = f.local(body[1]) else { return status(tx, req, E_INVAL) };
            let hc = f.hcs[k].as_mut().expect("a controller in place");
            let mut own = [0u8; 16];
            let own = &mut own[..body.len().min(16)];
            own.copy_from_slice(&body[..own.len()]);
            own[1] = local;
            hc.hold(slot);
            let n = hc.serve(req, own, tx);
            if i32::from_le_bytes([tx[HDR_LEN], tx[HDR_LEN + 1], tx[HDR_LEN + 2], tx[HDR_LEN + 3]])
                == 0
            {
                (f.bound[at], tx[DATA_AT + 1]) = (Bind::On(k), body[1]);
            } else {
                hc.release(slot);
            }
            n
        }
        (op, Some(Bind::On(k))) => {
            let n = f.hcs[k].as_mut().expect("a controller in place").serve(req, body, tx);
            if op == OP_DISABLE_SLOT {
                f.bound[at] = Bind::Free;
            }
            n
        }
        _ => status(tx, req, E_INVAL),
    }
}
