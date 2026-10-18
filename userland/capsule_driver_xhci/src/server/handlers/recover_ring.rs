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
use crate::controller::{recover_endpoint, EndpointAt};
use crate::dma::DmaRegion;
use crate::server::context::Context;

use super::recover::DCI_EP0;

/// Bring endpoint `dci` of `slot` back, then let go of `buf`, a request
/// buffer TRBs queued on it point at: freed once nothing queued before can
/// still run, or else kept with the ring until the controller passed them
/// (`TransferRing::keep`). An endpoint the slot does not have ran nothing.
pub(super) fn recover_ring(ctx: &mut Context, slot: u8, dci: u8, buf: Option<DmaRegion>) {
    let d = &mut ctx.driver;
    let context_size = d.layout.context_size;
    let Some(res) = d.slots.resources_mut(slot, d.layout.max_slots) else { return };
    let ring = if dci == DCI_EP0 {
        &res.ep0
    } else if let Some(ep) = res.interrupt.iter_mut().find(|ep| ep.dci == dci) {
        ep.armed = None;
        &ep.ring
    } else if let Some(pipes) = res.bulk.as_ref() {
        if dci == pipes.dci_in {
            &pipes.in_ring
        } else if dci == pipes.dci_out {
            &pipes.out_ring
        } else {
            return;
        }
    } else {
        return;
    };
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    let (cmd, evt) = (&mut d.command_ring, &mut d.event_ring);
    let ep = EndpointAt { ring, output: &res.output_context, context_size, slot, dci };
    if let (Err(_), Some(buf)) = (recover_endpoint(db, intr, cmd, evt, ep), buf) {
        ring.keep(buf);
    }
}
