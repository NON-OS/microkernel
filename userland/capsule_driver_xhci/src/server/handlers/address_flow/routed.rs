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
//! Address Device for a device below a hub: the hub has reset its own
//! port, so the root port is left alone (resetting it would reset the
//! hub), and the route and TT join the Slot Context before the command.

use super::alloc_resources::alloc_resources;
use super::complete_address::complete_address;
use crate::contexts::write_routed;
use crate::protocol::{Request, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;

/// Where the device sits below its root port.
pub struct Below {
    pub route: u32,
    /// The high-speed hub's slot and port, for a low or full speed device.
    pub tt: Option<(u8, u8)>,
}

pub fn address_routed(
    ctx: &mut Context,
    req: &Request,
    tx: &mut [u8],
    at: (u8, u8, u8),
    below: Below,
) {
    let (slot, port, speed) = at;
    let resources = match alloc_resources(ctx, slot, port, speed) {
        Ok(r) => r,
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    let csz = ctx.driver.layout.context_size;
    write_routed(&resources.input_context, csz, below.route, below.tt);
    complete_address(ctx, req, tx, resources);
}
