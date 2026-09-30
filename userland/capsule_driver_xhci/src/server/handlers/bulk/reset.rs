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
//! `OP_RESET_BULK` (`slot, dir_in`): recover a bulk endpoint that stalled.
use super::reply::send_prefix;
use crate::controller::reset_bulk_endpoint;
use crate::error::{XhciError, XhciResult};
use crate::protocol::{Request, E_INVAL, E_IO, RESET_BULK_REQUEST_LEN};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
pub fn reset(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != RESET_BULK_REQUEST_LEN || body[1] > 1 {
        return reply_with_status(tx, req, E_INVAL);
    }
    match run(ctx, body[0], body[1] == 1) {
        Ok(()) => send_prefix(tx, req, [0; 4]),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
fn run(ctx: &mut Context, slot: u8, dir_in: bool) -> XhciResult<()> {
    let max_slots = ctx.driver.layout.max_slots;
    let pipes = ctx
        .driver
        .slots
        .resources_mut(slot, max_slots)
        .and_then(|r| r.bulk.as_ref())
        .ok_or(XhciError::ControllerUnsupported)?;
    reset_bulk_endpoint(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.command_ring,
        &mut ctx.driver.event_ring,
        pipes,
        slot,
        dir_in,
    )
}
