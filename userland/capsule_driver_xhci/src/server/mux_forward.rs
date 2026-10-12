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

//! A request taken in: a claimant's slot made the parked one it stands
//! for, then carried to that slot's controller as its own slot; and the
//! status word of a reply kept back.

use alloc::vec::Vec;

use super::mux::{Mux, VSlot};
use crate::protocol::{Request, E_INVAL, RESP_HDR_LEN, STATUS_LEN};
use crate::server::dispatch::dispatch;
use crate::server::error::reply_with_status;

impl Mux {
    /// A request, a claimant's slot standing for the parked one it claimed.
    pub fn dispatch(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        if self.virtual_ops(req, body, tx) {
            return;
        }
        match body.first().map(|&s| self.virt.real(s)) {
            Some(real) if real != body[0] => {
                let mut own = Vec::from(body);
                own[0] = real;
                self.inner(req, &own, tx)
            }
            _ => self.inner(req, body, tx),
        }
    }

    /// A request whose first byte is a slot id, sent to that slot's controller.
    pub(super) fn forward(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        let Some(&v) = body.first() else { return reply_with_status(tx, req, E_INVAL) };
        let VSlot::Bound { ctrl, slot } = self.vslots[v as usize] else {
            return reply_with_status(tx, req, E_INVAL);
        };
        let mut local = Vec::from(body);
        local[0] = slot;
        dispatch(&mut self.ctxs[ctrl as usize], req, &local, tx);
    }
}

/// The status word of a reply `len` bytes long in `tx`.
pub(super) fn status(tx: &[u8], len: usize) -> Option<i32> {
    if len < RESP_HDR_LEN + STATUS_LEN {
        return None;
    }
    let at = tx.get(RESP_HDR_LEN..RESP_HDR_LEN + STATUS_LEN)?;
    Some(i32::from_le_bytes([at[0], at[1], at[2], at[3]]))
}
