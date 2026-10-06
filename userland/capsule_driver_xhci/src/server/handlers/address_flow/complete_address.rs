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

use super::super::address_reply::reply_ok;
use super::attach_resources::attach_resources;
use super::command_address::command_address;
use super::fix_ep0::fix_ep0;
use crate::controller::{clear_dcbaa_slot, set_dcbaa_slot};
use crate::protocol::{Request, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::SlotResources;

/// Install the output context, address the device, and settle EP0's max
/// packet size. A failure after the slot is in the table leaves it there:
/// the class driver's Disable Slot frees it once the controller has let go.
pub(super) fn complete_address(
    ctx: &mut Context,
    req: &Request,
    tx: &mut [u8],
    resources: SlotResources,
) {
    let (slot, port, speed) = (resources.slot_id, resources.port_id, resources.speed);
    let output_phys = resources.output_context.phys();
    if set_dcbaa_slot(&ctx.driver.dcbaa, slot, ctx.driver.layout.max_slots, output_phys).is_err() {
        reply_with_status(tx, req, E_IO);
        return;
    }
    if command_address(ctx, &resources).is_err() {
        let _ = clear_dcbaa_slot(&ctx.driver.dcbaa, slot, ctx.driver.layout.max_slots);
        reply_with_status(tx, req, E_IO);
        return;
    }
    if !attach_resources(ctx, resources) {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    match fix_ep0(ctx, slot) {
        Ok(mps) => reply_ok(tx, req, slot, port, speed, mps),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
