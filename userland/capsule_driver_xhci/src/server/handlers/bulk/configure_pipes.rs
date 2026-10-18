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
//! The Configure Endpoint that adds a slot's bulk pair, for `configure`.
use super::max_packet::bulk_max_packet;
use crate::contexts::is_superspeed;
use crate::controller::{issue_configure_endpoint, write_bulk_input, BulkEndpoint, BulkPipes};
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;
use crate::server::handlers::recover::keep_if_failed;

#[derive(Clone, Copy)]
pub(super) struct Bursts {
    pub(super) bulk_in: u8,
    pub(super) bulk_out: u8,
}

pub(super) fn run(
    ctx: &mut Context,
    (slot, second): (u8, bool),
    bulk_in: (u8, u16),
    bulk_out: (u8, u16),
    bursts: Bursts,
) -> XhciResult<()> {
    let pipes = BulkPipes::new(&ctx.driver.dma_pool, bulk_in.0, bulk_out.0)?;
    let context_size = ctx.driver.layout.context_size;
    let max_slots = ctx.driver.layout.max_slots;
    let res =
        ctx.driver.slots.resources_mut(slot, max_slots).ok_or(XhciError::ControllerUnsupported)?;
    // A Bluetooth controller's interrupt IN sits beside its bulk pair
    // (slots/admit.rs); a second pair takes endpoints the first has not.
    let fits =
        if second { res.takes_second_bulk((bulk_in.0, bulk_out.0)) } else { res.takes_bulk() };
    if !fits {
        return Err(XhciError::ControllerUnsupported);
    }
    let (speed, usb3) = (res.speed, res.usb3);
    let burst = |b: u8| if is_superspeed(speed, usb3) { b } else { 0 };
    let ep_in = BulkEndpoint {
        dci: bulk_in.0,
        ring_phys: pipes.in_ring.phys(),
        max_packet: bulk_max_packet(speed, usb3, bulk_in.1),
        max_burst: burst(bursts.bulk_in),
    };
    let ep_out = BulkEndpoint {
        dci: bulk_out.0,
        ring_phys: pipes.out_ring.phys(),
        max_packet: bulk_max_packet(speed, usb3, bulk_out.1),
        max_burst: burst(bursts.bulk_out),
    };
    write_bulk_input(&res.input_context, &res.output_context, context_size, ep_in, ep_out);
    let input_phys = res.input_context.phys();
    let (db, intr) = (ctx.driver.layout.doorbell_base, ctx.driver.layout.primary_intr_base);
    let (cmd, evt) = (&mut ctx.driver.command_ring, &mut ctx.driver.event_ring);
    if let Err(e) = issue_configure_endpoint(db, intr, cmd, evt, input_phys, slot) {
        return Err(keep_if_failed(ctx, slot, e, pipes.into_regions()));
    }
    if let Some(res) = ctx.driver.slots.resources_mut(slot, max_slots) {
        *(if second { &mut res.bulk2 } else { &mut res.bulk }) = Some(pipes);
    }
    Ok(())
}
