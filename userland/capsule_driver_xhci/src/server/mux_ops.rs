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

//! Slots across several controllers: Enable Slot reserves the
//! multiplexer's id (the controller's slot is enabled at Address Device,
//! mux_address.rs), and Disable Slot gives both back.

use super::mux::{status, Mux, VSlot};
use crate::protocol::{
    encode_response_header, write_status, Request, E_INVAL, E_NODEV, RESP_HDR_LEN,
    SLOT_DISABLE_PAYLOAD_LEN, SLOT_ENABLE_PAYLOAD_LEN, STATUS_LEN,
};
use crate::server::dispatch::dispatch;
use crate::server::error::reply_with_status;
use crate::server::reply::{captured, send};

impl Mux {
    pub(super) fn enable_slot(&mut self, req: &Request, tx: &mut [u8]) {
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

    pub(super) fn disable_slot(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
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
}
