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

//! Binding one of the multiplexer's slot ids to a controller's slot.

use super::mux::Mux;
use super::mux_forward::VSlot;
use super::mux_status::status;
use crate::protocol::{Request, E_INVAL, E_IO, OP_ENABLE_SLOT, RESP_HDR_LEN, STATUS_LEN};
use crate::server::dispatch::dispatch;
use crate::server::reply::{captured, sender};

impl Mux {
    /// Controller `ctrl`'s slot for `v`, enabled there now when the sender
    /// had only reserved `v`; the status to reply with otherwise. A bound
    /// slot's owner is held by its controller (`dispatch`).
    pub(super) fn bind(
        &mut self,
        v: u8,
        ctrl: usize,
        req: &Request,
        tx: &mut [u8],
    ) -> Result<u8, i32> {
        match self.vslots[v as usize] {
            VSlot::Bound { ctrl: c, slot } if c as usize == ctrl => Ok(slot),
            VSlot::Free | VSlot::Bound { .. } => Err(E_INVAL),
            VSlot::Pending { owner } if owner != sender() => Err(E_INVAL),
            VSlot::Pending { .. } => {
                let slot = self.enable_on(ctrl, req, tx).ok_or(E_IO)?;
                // The Enable Slot may have revived the controller first: the
                // ids bound to it before go now, so the next request does not
                // take this binding, made on the revived controller, as stale.
                self.forget_revived();
                self.vslots[v as usize] = VSlot::Bound { ctrl: ctrl as u8, slot };
                Ok(slot)
            }
        }
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
}
