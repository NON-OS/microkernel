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

//! The event ring, against a controller that writes whatever it likes.
//!
//! The controller owns every byte of an event TRB: the cycle bit, the type,
//! the completion code, the slot and endpoint ids, the residual length and
//! the 64-bit pointer. These proofs run the driver's own `EventRing` and
//! every function that consumes it (the interrupt drain, both completion
//! waits, the interrupt-IN poll, the bulk wait and poll, the isoch wait)
//! over host DMA memory, against a producer written from the specification.

pub(crate) mod events;
mod fixture;
mod fixture_slots;
pub(crate) mod producer;
mod producer_ring;
mod producer_start;
mod producer_write;

mod abort_failed_tests;
mod abort_ring_tests;
mod abort_unfetched_tests;
mod address_tests;
mod armed_place;
mod armed_tests;
mod bluetooth_limit_tests;
mod bluetooth_poll_tests;
mod bluetooth_tests;
mod boundary_pointer_tests;
mod boundary_tests;
mod boundary_through;
mod bulk_poll_more_tests;
mod bulk_poll_tests;
mod burst_tests;
mod command_at;
mod dequeue_start_tests;
mod dequeue_tests;
mod drain_batch_tests;
mod erdp_tests;
mod fuzz_tests;
mod hostile_tests;
mod isoch_pipe;
mod isoch_tests;
mod isoch_wait_tests;
mod match_bulk_tests;
mod match_command_tests;
mod match_control_tests;
mod match_enable_slot_tests;
mod match_impostors;
mod match_interrupt_tests;
mod match_stall_tests;
mod pair_admit_tests;
mod pair_tests;
mod parked_tests;
mod recover_refusal_tests;
mod recover_tests;
mod residual_bulk_tests;
mod residual_tests;
mod residual_wild_tests;
mod ring_full_tests;
mod stale_lap_tests;
mod stopped_tests;
mod stray_tests;
mod stream_tests;
mod stream_transfer_tests;
mod toggle_reset_tests;
mod toggle_slot;
mod toggle_tests;
