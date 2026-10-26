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
//! One endpoint given its streams (streams.rs): its stream contexts made,
//! the endpoint dropped and added again with them, then kept on its pair.

use crate::contexts::is_superspeed;
use crate::controller::{issue_configure_endpoint, write_stream_input, StreamPipe};
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;

/// The stream ids usable on it, 1 to the answer.
pub(super) fn give(
    ctx: &mut Context,
    (slot, second, dir_in): (u8, bool, bool),
    log2: u8,
) -> XhciResult<u16> {
    let (cs, max_slots) = (ctx.driver.layout.context_size, ctx.driver.layout.max_slots);
    let pipe = StreamPipe::new(&ctx.driver.dma_pool, log2)?;
    let refused = XhciError::ControllerUnsupported;
    let res = ctx.driver.slots.resources_mut(slot, max_slots).ok_or(refused)?;
    let superspeed = is_superspeed(res.speed, res.usb3);
    let p = res.pair_mut(second).ok_or(refused)?;
    let (dci, free) = match dir_in {
        true => (p.dci_in, p.in_streams.is_none() && p.armed.is_none()),
        false => (p.dci_out, p.out_streams.is_none()),
    };
    if !superspeed || !free {
        return Err(refused);
    }
    write_stream_input(&res.input_context, &res.output_context, cs, dci, &pipe);
    let input = res.input_context.phys();
    let (db, intr) = (ctx.driver.layout.doorbell_base, ctx.driver.layout.primary_intr_base);
    let d = &mut ctx.driver;
    issue_configure_endpoint(db, intr, &mut d.command_ring, &mut d.event_ring, input, slot)?;
    let usable = pipe.usable();
    let p = d.slots.resources_mut(slot, max_slots).and_then(|r| r.pair_mut(second));
    *(if dir_in {
        &mut p.ok_or(refused)?.in_streams
    } else {
        &mut p.ok_or(refused)?.out_streams
    }) = Some(pipe);
    Ok(usable)
}
