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

//! Address Device, and its routed form for a device below a hub, across
//! controllers: the root port names the controller, a slot id only
//! reserved is enabled there (`mux_bind`), and the reply names the slot
//! and port as the class driver knows them. A routed request is held
//! against that controller's hubs first (`mux_routed`).

use alloc::vec::Vec;

use super::mux::Mux;
use super::mux_ports::local_port;
use super::mux_status::status;
use crate::protocol::{
    Request, ADDRESS_DEVICE_REQUEST_LEN, ADDRESS_ROUTED_REQUEST_LEN, E_INVAL, OP_ADDRESS_ROUTED,
    RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::dispatch::dispatch;
use crate::server::error::reply_with_status;
use crate::server::reply::{captured, send};

impl Mux {
    pub(super) fn address(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        let routed = req.op == OP_ADDRESS_ROUTED;
        let want = if routed { ADDRESS_ROUTED_REQUEST_LEN } else { ADDRESS_DEVICE_REQUEST_LEN };
        if body.len() != want {
            return reply_with_status(tx, req, E_INVAL);
        }
        let (v, port) = (body[0], body[1]);
        let Some((ctrl, lport)) = local_port(&self.ports, port) else {
            return reply_with_status(tx, req, E_INVAL);
        };
        let local = if routed { self.routed_local(ctrl, lport, body) } else { Ok(Vec::from(body)) };
        let Ok(mut local) = local else { return reply_with_status(tx, req, E_INVAL) };
        let slot = match self.bind(v, ctrl, req, tx) {
            Ok(slot) => slot,
            Err(e) => return reply_with_status(tx, req, e),
        };
        (local[0], local[1]) = (slot, lport);
        let ctx = &mut self.ctxs[ctrl];
        let len = captured(|| dispatch(ctx, req, &local, tx));
        if status(tx, len) == Some(0) && len >= RESP_HDR_LEN + STATUS_LEN + 2 {
            tx[RESP_HDR_LEN + STATUS_LEN] = v;
            tx[RESP_HDR_LEN + STATUS_LEN + 1] = port;
        }
        send(tx.as_ptr(), len);
    }
}
