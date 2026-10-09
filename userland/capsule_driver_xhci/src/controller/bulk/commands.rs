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
//! The two commands that bring a halted bulk endpoint back: Reset Endpoint
//! and Set TR Dequeue Pointer (xHCI 1.2 sections 6.4.3.7 and 6.4.3.9).
use crate::trb::Trb;
const TRB_TYPE_RESET_ENDPOINT_CMD: u32 = 14;
const TRB_TYPE_SET_TR_DEQUEUE_CMD: u32 = 16;
fn endpoint_command(ty: u32, slot: u8, dci: u8, cycle: bool) -> Trb {
    let mut trb = Trb::zero();
    trb.set_type(ty);
    trb.d3 |= ((dci as u32) << 16) | ((slot as u32) << 24);
    trb.set_cycle(cycle);
    trb
}
pub(super) fn reset_endpoint_command(slot: u8, dci: u8, cycle: bool) -> Trb {
    endpoint_command(TRB_TYPE_RESET_ENDPOINT_CMD, slot, dci, cycle)
}
/// Move the endpoint's dequeue pointer to `dequeue_phys`, where the next
/// TRB will be written, with `ring_cycle` as its consumer cycle state.
pub(super) fn set_dequeue_command(
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
