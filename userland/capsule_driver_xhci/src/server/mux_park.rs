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

//! OP_PARK: a child driver.usb_hub0 addressed below a hub, parked for a
//! class driver to claim. Only a driver admitted on both the child's slot
//! and the hub's parks it (slots/gate.rs), so no other capsule takes an
//! addressed device's port out from under its driver; the child and its
//! hub must be addressed on one controller, and the parent a hub.

use nonos_libc::mk_pid_alive;
use nonos_usb_virt::parse_park;

use super::mux::Mux;
use crate::protocol::{Request, E_BUSY, E_INVAL};
use crate::server::error::reply_with_status;
use crate::server::reply::sender;
use crate::slots::gate_park;

impl Mux {
    pub(super) fn park(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) {
        let Some(p) = parse_park(body) else { return reply_with_status(tx, req, E_INVAL) };
        let alive = |q| mk_pid_alive(q);
        if let Err(e) = gate_park(&mut self.share, p.slot, p.parent, sender(), alive) {
            return reply_with_status(tx, req, e);
        }
        let (Some((c, _)), Some((pc, pl))) = (self.addressed(p.slot), self.addressed(p.parent))
        else {
            return reply_with_status(tx, req, E_INVAL);
        };
        if c != pc || !self.ctxs[c].driver.slots.is_hub(pl) || self.virt.knows(p.parent) {
            return reply_with_status(tx, req, E_INVAL);
        }
        let known = self.virt.knows(p.slot);
        match self.virt.park(p) {
            Some(_) => reply_with_status(tx, req, 0),
            None => reply_with_status(tx, req, if known { E_INVAL } else { E_BUSY }),
        }
    }
}
