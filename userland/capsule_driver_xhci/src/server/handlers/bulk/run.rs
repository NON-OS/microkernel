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
//! Moving one bulk transfer's bytes through the slot's DMA page.
use crate::controller::bulk_transfer;
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;
/// OUT when `into` is `None`, else IN with the bytes copied into `into`.
pub(super) fn run(
    ctx: &mut Context,
    slot: u8,
    data: &[u8],
    len: u32,
    into: Option<&mut [u8]>,
) -> XhciResult<u32> {
    let (doorbell, intr) = (ctx.driver.layout.doorbell_base, ctx.driver.layout.primary_intr_base);
    let max_slots = ctx.driver.layout.max_slots;
    let pipes = ctx
        .driver
        .slots
        .resources_mut(slot, max_slots)
        .and_then(|r| r.bulk.as_mut())
        .ok_or(XhciError::ControllerUnsupported)?;
    // No transfer is outstanding on the slot's page while this one is set
    // up or read back.
    if !pipes.buf.copy_from(data) {
        return Err(XhciError::ControllerUnsupported);
    }
    let dir_in = into.is_some();
    let moved =
        bulk_transfer(doorbell, intr, &mut ctx.driver.event_ring, pipes, slot, dir_in, len)?;
    if let Some(out) = into {
        let n = (moved as usize).min(out.len());
        pipes.buf.copy_to(&mut out[..n]);
    }
    Ok(moved)
}
