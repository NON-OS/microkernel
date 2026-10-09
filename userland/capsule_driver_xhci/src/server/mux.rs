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

//! Several xHCI controllers behind the one `driver.xhci0` endpoint. A
//! laptop's chipset controller may sit beside a Thunderbolt or USB4 one, and
//! AMD Ryzen APUs have two, each with its own root ports; serving only the
//! first left every port of the others dead. The class drivers keep the
//! wire format they have: root ports are numbered across the controllers
//! (the primary's from 1, the next one's after its last), and the slot id a
//! class driver holds is the multiplexer's own. Enable Slot only reserves
//! such an id; the controller's slot is enabled at Address Device, once the
//! port, and so the controller, is known. With one controller every request
//! goes to it unchanged.

use alloc::vec::Vec;

use super::mux_ports::{local_port, port_ranges};
use crate::protocol::{
    encode_response_header, write_status, Request, ADDRESS_DEVICE_REQUEST_LEN, E_INVAL, E_IO,
    E_NODEV, OP_ADDRESS_DEVICE, OP_CONTROLLER_STATUS, OP_DISABLE_SLOT, OP_ENABLE_SLOT,
    OP_HEALTHCHECK, OP_PORT_STATUS, PORT_ENTRY_BYTES, PORT_STATUS_HEADER_BYTES, RESP_HDR_LEN,
    SLOT_DISABLE_PAYLOAD_LEN, SLOT_ENABLE_PAYLOAD_LEN, STATUS_LEN,
};
use crate::regs::op::{portsc_clear_changes, portsc_read};
use crate::server::context::Context;
use crate::server::dispatch::dispatch;
use crate::server::error::reply_with_status;
use crate::server::reply::{captured, send};

#[derive(Clone, Copy, PartialEq, Eq)]
enum VSlot {
    Free,
    /// Reserved by Enable Slot, no controller yet.
    Pending,
    Bound {
        ctrl: u8,
        slot: u8,
    },
}

pub struct Mux {
    pub ctxs: Vec<Context>,
    /// Each controller's first global port number less one, and its port count.
    ports: Vec<(u8, u8)>,
    vslots: [VSlot; 256],
}

impl Mux {
    pub fn new(ctxs: Vec<Context>) -> Self {
        let counts: Vec<u8> = ctxs.iter().map(|c| c.driver.layout.max_ports).collect();
        Self { ports: port_ranges(&counts), ctxs, vslots: [VSlot::Free; 256] }
    }

    pub fn dispatch(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        if self.ctxs.len() == 1 {
            return dispatch(&mut self.ctxs[0], req, body, tx);
        }
        match req.op {
            OP_HEALTHCHECK | OP_CONTROLLER_STATUS => dispatch(&mut self.ctxs[0], req, body, tx),
            OP_PORT_STATUS if body.is_empty() => self.port_status(req, tx),
            OP_ENABLE_SLOT if body.is_empty() => self.enable_slot(req, tx),
            OP_DISABLE_SLOT => self.disable_slot(req, body, tx),
            OP_ADDRESS_DEVICE => self.address_device(req, body, tx),
            _ => self.forward(req, body, tx),
        }
    }

    /// A request whose first byte is a slot id, sent to that slot's controller.
    fn forward(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        let Some(&v) = body.first() else { return reply_with_status(tx, req, E_INVAL) };
        let VSlot::Bound { ctrl, slot } = self.vslots[v as usize] else {
            return reply_with_status(tx, req, E_INVAL);
        };
        let mut local = Vec::from(body);
        local[0] = slot;
        dispatch(&mut self.ctxs[ctrl as usize], req, &local, tx);
    }

    fn enable_slot(&mut self, req: &Request, tx: &mut [u8]) {
        let Some(v) = (1..=255u8).find(|&v| self.vslots[v as usize] == VSlot::Free) else {
            return reply_with_status(tx, req, E_NODEV);
        };
        self.vslots[v as usize] = VSlot::Pending;
        let payload_len = (STATUS_LEN + SLOT_ENABLE_PAYLOAD_LEN) as u32;
        encode_response_header(tx, req, payload_len);
        write_status(&mut tx[RESP_HDR_LEN..], 0);
        tx[RESP_HDR_LEN + STATUS_LEN] = v;
        tx[RESP_HDR_LEN + STATUS_LEN + 1..RESP_HDR_LEN + STATUS_LEN + 4].fill(0);
        send(tx.as_ptr(), RESP_HDR_LEN + payload_len as usize);
    }

    fn disable_slot(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        if body.len() != SLOT_DISABLE_PAYLOAD_LEN {
            return reply_with_status(tx, req, E_INVAL);
        }
        let v = body[0] as usize;
        match self.vslots[v] {
            VSlot::Free => reply_with_status(tx, req, E_INVAL),
            VSlot::Pending => {
                self.vslots[v] = VSlot::Free;
                reply_with_status(tx, req, 0);
            }
            VSlot::Bound { ctrl, slot } => {
                let ctx = &mut self.ctxs[ctrl as usize];
                let len = captured(|| dispatch(ctx, req, &[slot], tx));
                if status(tx, len) == Some(0) {
                    self.vslots[v] = VSlot::Free;
                }
                send(tx.as_ptr(), len);
            }
        }
    }

    fn address_device(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
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
    fn enable_on(&mut self, ctrl: usize, req: &Request, tx: &mut [u8]) -> Option<u8> {
        let enable = Request { op: OP_ENABLE_SLOT, payload_len: 0, ..*req };
        let ctx = &mut self.ctxs[ctrl];
        let len = captured(|| dispatch(ctx, &enable, &[], tx));
        if status(tx, len) != Some(0) || len <= RESP_HDR_LEN + STATUS_LEN {
            return None;
        }
        Some(tx[RESP_HDR_LEN + STATUS_LEN])
    }

    fn port_status(&mut self, req: &Request, tx: &mut [u8]) {
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

/// The status word of a reply `len` bytes long in `tx`.
fn status(tx: &[u8], len: usize) -> Option<i32> {
    if len < RESP_HDR_LEN + STATUS_LEN {
        return None;
    }
    let at = tx.get(RESP_HDR_LEN..RESP_HDR_LEN + STATUS_LEN)?;
    Some(i32::from_le_bytes([at[0], at[1], at[2], at[3]]))
}
