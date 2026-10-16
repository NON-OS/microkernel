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

//! A request that binds a device (slots/binds.rs) weighed against the
//! device's standing (slots/authorize.rs). A device not yet judged is
//! judged by its first configuration, read here through the same path as
//! a class driver's read; under Allow nothing is read and nothing changes.

use alloc::vec::Vec;

use nonos_libc::mk_debug;

use super::mux::{status, Mux};
use crate::controller::CONFIG_DESCRIPTOR_MAX;
use crate::protocol::{
    Request, CONFIG_DESCRIPTOR_REPLY_PREFIX, OP_GET_CONFIG_DESCRIPTOR, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::error::reply_with_status;
use crate::server::reply::captured;
use crate::slots::binds::binds;
use crate::slots::Standing;

const HELD: &[u8] = b"xhci: a device is held for the owner's answer (OP_USB_HELD)";

impl Mux {
    /// True when the bind was refused and answered.
    pub(super) fn bind_refused(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
        let Some(&slot) = body.first().filter(|_| binds(req.op, body)) else { return false };
        if self.share.auth.admit(slot).is_none() {
            let config = self.first_config(req, slot, tx);
            if let Standing::Held { .. } = self.share.auth.judge(slot, config.as_deref()) {
                let _ = mk_debug(HELD.as_ptr(), HELD.len());
            }
        }
        match self.share.auth.admit(slot) {
            Some(Err(e)) => {
                reply_with_status(tx, req, e);
                true
            }
            _ => false,
        }
    }

    fn first_config(&mut self, req: &Request, slot: u8, tx: &mut [u8]) -> Option<Vec<u8>> {
        let [lo, hi] = CONFIG_DESCRIPTOR_MAX.to_le_bytes();
        let read = Request { op: OP_GET_CONFIG_DESCRIPTOR, payload_len: 4, ..*req };
        let len = captured(|| self.inner(&read, &[slot, 0, lo, hi], tx));
        if status(tx, len) != Some(0) {
            return None;
        }
        let at = RESP_HDR_LEN + STATUS_LEN;
        let n = u16::from_le_bytes([*tx.get(at)?, *tx.get(at + 1)?]) as usize;
        let from = at + CONFIG_DESCRIPTOR_REPLY_PREFIX;
        tx.get(from..from + n).map(|b| b.to_vec())
    }
}
