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

//! The driver against fake DMA memory, a model register file and a hostile
//! device; and the code rules over its source.

mod common;
mod drive;
mod fuzz;
mod harness;
mod mem;
mod space;

mod choose_tests;
mod features_tests;
mod grant_tests;
mod legacy_queue_tests;
mod limits_tests;
mod modern_queue_tests;
mod order_tests;
mod rules_tests;
mod rx_batch_tests;
mod rx_bound_tests;
mod rx_copy_tests;
mod rx_edge_tests;
mod rx_fuzz_tests;
mod rx_packet_tests;
mod rx_reply_tests;
mod rx_resend_tests;
mod transport_tests;
mod tx_gate_tests;
mod tx_hostile_tests;
mod tx_reset_tests;
mod tx_ring_tests;
mod tx_stall_tests;
mod wire_tests;
