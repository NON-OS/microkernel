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

//! The multiplexer's slot ids, and a request sent on by one.

use alloc::vec::Vec;

use super::mux::Mux;
use crate::protocol::{Request, E_INVAL};
use crate::server::dispatch::dispatch;
use crate::server::error::reply_with_status;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum VSlot {
    Free,
    /// Reserved by Enable Slot for client `owner`, no controller yet.
    Pending {
        owner: u32,
    },
    Bound {
        ctrl: u8,
        slot: u8,
    },
}

impl Mux {
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

    /// A controller brought back since its slots were bound dropped every
    /// slot it had: the ids bound to it name nothing now, and are free.
    pub(super) fn forget_revived(&mut self) {
        for (i, ctx) in self.ctxs.iter().enumerate() {
            if self.generations[i] == ctx.driver.generation {
                continue;
            }
            self.generations[i] = ctx.driver.generation;
            for v in self.vslots.iter_mut() {
                if matches!(*v, VSlot::Bound { ctrl, .. } if ctrl as usize == i) {
                    *v = VSlot::Free;
                }
            }
        }
    }
}
