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
/// Whether the endpoint is back: false when it could not be brought back,
/// or when the slot has no such endpoint to bring back.
pub(super) fn recover_ring(ctx: &mut Context, slot: u8, dci: u8, buf: Option<DmaRegion>) -> bool {
    let d = &mut ctx.driver;
    let context_size = d.layout.context_size;
    let Some(res) = d.slots.resources_mut(slot, d.layout.max_slots) else { return false };
    let ring = if dci == DCI_EP0 {
        &res.ep0
    } else if let Some(ep) = res.interrupt.iter_mut().find(|ep| ep.dci == dci) {
        ep.armed = None;
        &ep.ring
    } else if let Some(p) =
        [res.isoch.as_mut(), res.isoch_in.as_mut()].into_iter().flatten().find(|p| p.ep.dci == dci)
    {
        p.forget();
        &p.ring
    } else if let Some(pipes) = [res.bulk.as_mut(), res.bulk2.as_mut()]
        .into_iter()
        .flatten()
        .find(|p| p.ring_of(dci).is_some())
    {
        // A polled IN's TRB is given up with the rest: the next look arms anew.
        if dci == pipes.dci_in {
            pipes.armed = None;
        }
        pipes.ring_of(dci).unwrap_or(&pipes.out_ring)
    } else {
        return false;
    };
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    let (cmd, evt) = (&mut d.command_ring, &mut d.event_ring);
    let ep = EndpointAt { ring, output: &res.output_context, context_size, slot, dci };
    let recovered = recover_endpoint(db, intr, cmd, evt, ep);
    if let (Err(_), Some(buf)) = (&recovered, buf) {
        ring.keep(buf);
    }
    recovered.is_ok()
}
