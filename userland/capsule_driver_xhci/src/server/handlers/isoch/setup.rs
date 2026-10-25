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

//! The endpoint a configure request adds: its ring and buffers lent from
//! the pool, its input context, Configure Endpoint, and the pipe kept in the
//! slot by direction.

use crate::contexts::is_superspeed;
use crate::controller::{
    isoch_endpoint, issue_configure_endpoint, write_isoch_input, IsochEndpoint, IsochPipe,
};
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;
use crate::server::handlers::recover::keep_if_failed;
use crate::slots::SlotResources;

pub(super) fn run(
    ctx: &mut Context,
    slot: u8,
    dci: u8,
    b_interval: u8,
    ss: (u8, u8),
    mps: u16,
) -> XhciResult<()> {
    let d = &mut ctx.driver;
    let (context_size, max_slots) = (d.layout.context_size, d.layout.max_slots);
    let res = d.slots.resources_mut(slot, max_slots).ok_or(XhciError::ControllerUnsupported)?;
    if pipe_of(res, dci).is_some() || mps & 0x07FF == 0 {
        return Err(XhciError::ControllerUnsupported);
    }
    // Allocated only for a request that will be served: the pool lends once.
    let (ring, buf) = IsochPipe::memory(&d.dma_pool)?;
    let ep = isoch_endpoint(res.speed, res.usb3, dci, b_interval, ss, mps);
    let ep = IsochEndpoint { ring_phys: ring.phys(), ..ep };
    write_isoch_input(&res.input_context, &res.output_context, context_size, ep);
    let (input_phys, superspeed) = (res.input_context.phys(), is_superspeed(res.speed, res.usb3));
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    let (cmd, evt) = (&mut d.command_ring, &mut d.event_ring);
    if let Err(e) = issue_configure_endpoint(db, intr, cmd, evt, input_phys, slot) {
        // A failed controller may have taken the ring; it stays with the slot.
        return Err(keep_if_failed(ctx, slot, e, ring.into_regions().chain([buf])));
    }
    if let Some(res) = ctx.driver.slots.resources_mut(slot, max_slots) {
        *pipe_of(res, dci) = Some(IsochPipe::with(ring, buf, ep, superspeed));
    }
    Ok(())
}

/// An odd DCI is an IN endpoint.
fn pipe_of(res: &mut SlotResources, dci: u8) -> &mut Option<IsochPipe> {
    if dci & 1 == 1 {
        &mut res.isoch_in
    } else {
        &mut res.isoch
    }
}
