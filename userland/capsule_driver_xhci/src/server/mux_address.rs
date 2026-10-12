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

//! Address Device across several controllers: the port names the
//! controller, the slot is enabled there, and the reply carries the
//! multiplexer's slot id and port back.

use super::mux::{status, Mux, VSlot};
use super::mux_ports::local_port;
use crate::protocol::{
    Request, ADDRESS_DEVICE_REQUEST_LEN, E_INVAL, E_IO, OP_ENABLE_SLOT, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::dispatch::dispatch;
use crate::server::error::reply_with_status;
use crate::server::reply::{captured, send};

impl Mux {
    pub(super) fn address_device(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        if body.len() != ADDRESS_DEVICE_REQUEST_LEN {
            return reply_with_status(tx, req, E_INVAL);
        }
        let (v, port) = (body[0], body[1]);
        let Some((ctrl, lport)) = local_port(&self.ports, port) else {
            return reply_with_status(tx, req, E_INVAL);
        };
        let slot = match self.vslots[v as usize] {
            VSlot::Free => return reply_with_status(tx, req, E_INVAL),
            VSlot::Bound { ctrl: c, slot } if c as usize == ctrl => slot,
            VSlot::Bound { .. } => return reply_with_status(tx, req, E_INVAL),
            VSlot::Pending => match self.enable_on(ctrl, req, tx) {
                Some(slot) => {
                    self.vslots[v as usize] = VSlot::Bound { ctrl: ctrl as u8, slot };
                    slot
                }
                None => return reply_with_status(tx, req, E_IO),
            },
        };
        let ctx = &mut self.ctxs[ctrl];
        let len = captured(|| dispatch(ctx, req, &[slot, lport], tx));
        if status(tx, len) == Some(0) && len >= RESP_HDR_LEN + STATUS_LEN + 2 {
            tx[RESP_HDR_LEN + STATUS_LEN] = v;
            tx[RESP_HDR_LEN + STATUS_LEN + 1] = port;
        }
        send(tx.as_ptr(), len);
    }

    /// Enable a slot on controller `ctrl` through its own handler.
    pub(super) fn enable_on(&mut self, ctrl: usize, req: &Request, tx: &mut [u8]) -> Option<u8> {
        let enable = Request { op: OP_ENABLE_SLOT, payload_len: 0, ..*req };
        let ctx = &mut self.ctxs[ctrl];
        let len = captured(|| dispatch(ctx, &enable, &[], tx));
        if status(tx, len) != Some(0) || len <= RESP_HDR_LEN + STATUS_LEN {
            return None;
        }
        Some(tx[RESP_HDR_LEN + STATUS_LEN])
    }
}
