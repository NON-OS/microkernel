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

//! The commands that act on one endpoint: Reset Endpoint, Stop Endpoint and
//! Set TR Dequeue Pointer (xHCI 1.2 sections 6.4.3.7, 6.4.3.8 and 6.4.3.9),
//! and Evaluate Context (6.4.3.6), which names a slot and an input context.
use crate::trb::Trb;
const TRB_TYPE_EVALUATE_CONTEXT_CMD: u32 = 13;
const TRB_TYPE_RESET_ENDPOINT_CMD: u32 = 14;
const TRB_TYPE_STOP_ENDPOINT_CMD: u32 = 15;
const TRB_TYPE_SET_TR_DEQUEUE_CMD: u32 = 16;
fn endpoint_command(ty: u32, slot: u8, dci: u8, cycle: bool) -> Trb {
    let mut trb = Trb::zero();
    trb.set_type(ty);
    trb.d3 |= ((dci as u32) << 16) | ((slot as u32) << 24);
    trb.set_cycle(cycle);
    trb
}
/// Reset Endpoint with TSP clear: the endpoint leaves Halted for Stopped
/// and its data toggle or sequence number goes back to zero.
pub fn reset_endpoint_command(slot: u8, dci: u8, cycle: bool) -> Trb {
    endpoint_command(TRB_TYPE_RESET_ENDPOINT_CMD, slot, dci, cycle)
}
/// Stop Endpoint: a Running endpoint stops where it is, so its ring can be
/// moved past TRBs that will never complete.
pub fn stop_endpoint_command(slot: u8, dci: u8, cycle: bool) -> Trb {
    endpoint_command(TRB_TYPE_STOP_ENDPOINT_CMD, slot, dci, cycle)
}
/// Move the endpoint's dequeue pointer to `dequeue_phys`, where the next
/// TRB will be written, with `ring_cycle` as its consumer cycle state.
pub fn set_dequeue_command(
    slot: u8,
    dci: u8,
    dequeue_phys: u64,
    ring_cycle: bool,
    cycle: bool,
) -> Trb {
    let mut trb = endpoint_command(TRB_TYPE_SET_TR_DEQUEUE_CMD, slot, dci, cycle);
    trb.set_pointer((dequeue_phys & !0xF) | ring_cycle as u64);
    trb
}
/// Evaluate Context for `slot` from the input context at `input_context_phys`.
pub fn evaluate_context_command(input_context_phys: u64, slot: u8, cycle: bool) -> Trb {
    let mut trb = Trb::zero();
    trb.set_pointer(input_context_phys & !0xF);
    trb.set_type(TRB_TYPE_EVALUATE_CONTEXT_CMD);
    trb.d3 |= (slot as u32) << 24;
    trb.set_cycle(cycle);
    trb
}
