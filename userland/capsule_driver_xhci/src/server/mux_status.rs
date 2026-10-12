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

//! The port list across several controllers, numbered on.

use super::mux::Mux;
use crate::protocol::{
    encode_response_header, write_status, Request, PORT_ENTRY_BYTES, PORT_STATUS_HEADER_BYTES,
    RESP_HDR_LEN, STATUS_LEN,
};
use crate::regs::op::{portsc_clear_changes, portsc_read};
use crate::server::reply::send;

impl Mux {
    pub(super) fn port_status(&mut self, req: &Request, tx: &mut [u8]) {
        let total: usize = self.ports.iter().map(|&(_, n)| n as usize).sum();
        let payload_bytes = STATUS_LEN + PORT_STATUS_HEADER_BYTES + total * PORT_ENTRY_BYTES;
        encode_response_header(tx, req, payload_bytes as u32);
        write_status(&mut tx[RESP_HDR_LEN..], 0);
        let mut o = RESP_HDR_LEN + STATUS_LEN;
        tx[o..o + PORT_STATUS_HEADER_BYTES].fill(0);
        tx[o] = total as u8;
        o += PORT_STATUS_HEADER_BYTES;
        for (ctx, &(base, count)) in self.ctxs.iter().zip(self.ports.iter()) {
            for lport in 1..=count {
                let portsc = portsc_read(ctx.driver.layout.op_base, lport);
                portsc_clear_changes(ctx.driver.layout.op_base, lport, portsc);
                tx[o] = base + lport;
                tx[o + 1] = ctx.driver.slots.port_state(lport);
                tx[o + 2] = 0;
                tx[o + 3] = 0;
                tx[o + 4..o + 8].copy_from_slice(&portsc.to_le_bytes());
                o += PORT_ENTRY_BYTES;
            }
        }
        send(tx.as_ptr(), RESP_HDR_LEN + payload_bytes);
    }
}
