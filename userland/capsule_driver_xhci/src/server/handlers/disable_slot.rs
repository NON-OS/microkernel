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
use crate::controller::{clear_dcbaa_slot, issue_disable_slot};
use crate::error::XhciResult;
use crate::protocol::{Request, E_INVAL, E_IO, SLOT_DISABLE_PAYLOAD_LEN};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != SLOT_DISABLE_PAYLOAD_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let slot_id = body[0];
    if !ctx.driver.slots.is_allocated(slot_id, ctx.driver.layout.max_slots) {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let status = if release_slot(ctx, slot_id).is_ok() { 0 } else { E_IO };
    reply_with_status(tx, req, status);
}

/// Disable the allocated `slot_id`. Disabled, its contexts, rings and kept
/// buffers are the driver's again: the DCBAA entry goes, then they are
/// freed, and no event kept for one of its TRBs answers a TRB the id's
/// next device has at the same address. A slot the controller did not
/// disable keeps all of it.
pub fn release_slot(ctx: &mut Context, slot_id: u8) -> XhciResult<()> {
    let d = &mut ctx.driver;
    let (db, intr, max) = (d.layout.doorbell_base, d.layout.primary_intr_base, d.layout.max_slots);
    issue_disable_slot(db, intr, &mut d.command_ring, &mut d.event_ring, slot_id)?;
    d.event_ring.forget_slot(slot_id);
    if d.slots.has_resources(slot_id) {
        let _ = clear_dcbaa_slot(&d.dcbaa, slot_id, max);
        let _ = d.slots.take_resources(slot_id, max);
    }
    let _ = d.slots.mark_released(slot_id, max);
    Ok(())
}
