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
//! OP_CONFIGURE_HUB: `slot, ports, think_time, mtt`. The slot's context is
//! marked a hub through Configure Endpoint with only the Slot Context
//! added, so the controller schedules split transactions below it; its
//! endpoints stay as they are. Reply: status only.

use crate::contexts::write_hub_input;
use crate::controller::issue_configure_endpoint;
use crate::protocol::{Request, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;

/// A USB 2.0 TT think time is 8 to 32 FS bit times, coded 0 to 3.
const THINK_MAX: u8 = 3;

pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let &[slot, ports, think, mtt @ (0 | 1)] = body else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let d = &mut ctx.driver;
    let (db, intr, csz) =
        (d.layout.doorbell_base, d.layout.primary_intr_base, d.layout.context_size);
    let Some(res) = d.slots.resources_mut(slot, d.layout.max_slots) else {
        return reply_with_status(tx, req, E_INVAL);
    };
    if ports == 0 || think > THINK_MAX {
        return reply_with_status(tx, req, E_INVAL);
    }
    write_hub_input(&res.input_context, &res.output_context, csz, (ports, think, mtt == 1));
    let input = res.input_context.phys();
    match issue_configure_endpoint(db, intr, &mut d.command_ring, &mut d.event_ring, input, slot) {
        Ok(()) => {
            res.hub = true;
            reply_with_status(tx, req, 0)
        }
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
