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

//! OP_ADDRESS_ROUTED across several controllers: the root port names the
//! controller, and the TT's hub slot, a slot id of the multiplexer's like
//! the device's own, must be on that controller; both go to it as its own
//! numbers, and the reply carries the multiplexer's back.

use alloc::vec::Vec;

use super::mux::{status, Mux, VSlot};
use super::mux_ports::local_port;
use crate::protocol::{Request, E_INVAL, E_IO, RESP_HDR_LEN, STATUS_LEN};
use crate::server::dispatch::dispatch;
use crate::server::error::reply_with_status;
use crate::server::reply::{captured, send};

impl Mux {
    pub(super) fn address_routed(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        let (Some(&v), Some(&port), Some(&tt)) = (body.first(), body.get(1), body.get(7)) else {
            return reply_with_status(tx, req, E_INVAL);
        };
        let Some((ctrl, lport)) = local_port(&self.ports, port) else {
            return reply_with_status(tx, req, E_INVAL);
        };
        let tt_local = match self.vslots[tt as usize] {
            _ if tt == 0 => 0,
            VSlot::Bound { ctrl: c, slot } if c as usize == ctrl => slot,
            _ => return reply_with_status(tx, req, E_INVAL),
        };
        let slot = match self.vslots[v as usize] {
            VSlot::Pending => match self.enable_on(ctrl, req, tx) {
                Some(slot) => {
                    self.vslots[v as usize] = VSlot::Bound { ctrl: ctrl as u8, slot };
                    slot
                }
                None => return reply_with_status(tx, req, E_IO),
            },
            _ => return reply_with_status(tx, req, E_INVAL),
        };
        let mut local = Vec::from(body);
        (local[0], local[1], local[7]) = (slot, lport, tt_local);
        let ctx = &mut self.ctxs[ctrl];
        let len = captured(|| dispatch(ctx, req, &local, tx));
        if status(tx, len) == Some(0) && len >= RESP_HDR_LEN + STATUS_LEN + 2 {
            tx[RESP_HDR_LEN + STATUS_LEN] = v;
            tx[RESP_HDR_LEN + STATUS_LEN + 1] = port;
        }
        send(tx.as_ptr(), len);
    }
}
