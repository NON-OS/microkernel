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

//! Resetting an endpoint's data toggle or sequence number on the host side
//! when Reset Endpoint did not: Configure Endpoint with the endpoint dropped
//! and added again (`write_endpoint_reset_input`), as Linux's
//! `xhci_endpoint_reset` does. The class driver clears the device's halt
//! feature right after, which resets the device's side.

use super::run_command::run_command;
use crate::contexts::{write_endpoint_reset_input, Dequeue};
use crate::dma::DmaRegion;
use crate::error::{XhciError, XhciResult};
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::rings::transfer::TransferRing;
use crate::trb::commands::configure_endpoint_command;

/// The endpoint whose toggle goes back to zero, and the slot contexts it is
/// described in.
pub struct ToggleReset<'a> {
    pub input: &'a DmaRegion,
    pub output: &'a DmaRegion,
    pub context_size: u8,
    pub ring: &'a TransferRing,
    pub slot: u8,
    pub dci: u8,
}

pub fn reset_toggle(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    ep: ToggleReset,
) -> XhciResult<()> {
    let dequeue = Dequeue { phys: ep.ring.enqueue_phys(), cycle: ep.ring.cycle() != 0 };
    write_endpoint_reset_input(ep.input, ep.output, ep.context_size, ep.dci, dequeue);
    let trb = configure_endpoint_command(ep.input.phys(), ep.slot, cmd_ring.cycle() != 0);
    let completion = run_command(doorbell_base, intr_base, cmd_ring, evt_ring, trb)?;
    if completion.slot_id != ep.slot {
        return Err(XhciError::UnexpectedCompletionSlot);
    }
    evt_ring.forget_parked(ep.slot, ep.dci);
    Ok(())
}
