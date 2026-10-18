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

//! The tests, one concern per file, and the helpers they share.

mod bring_up_fail_tests;
mod bring_up_flush_tests;
mod bring_up_k1_tests;
mod bring_up_pch_tests;
mod bring_up_tests;
mod bus_master_tests;
mod discover_tests;
mod flush_master_tests;
mod flush_stop_tests;
mod flush_tests;
mod helpers;
mod ids_scope_tests;
mod ids_tests;
mod interconnect_tests;
mod interconnect_toggle_tests;
mod log_tests;
mod mdic_tests;
mod order;
mod phy_access_tests;
mod predecessor_tests;
mod probe;
mod protocol_tests;
mod quiesce_tests;
mod reset_fail_tests;
mod reset_tests;
mod rollback_master_tests;
mod rollback_tests;
mod runner_tests;
mod rx_batch_tests;
mod rx_handler_tests;
mod rx_ring_tests;
mod rx_tests;
mod serve;
mod swflag_tests;
mod teardown_flush_tests;
mod tx_handler_tests;
mod tx_hang_fail_tests;
mod tx_hang_mac_tests;
mod tx_hang_tests;
mod tx_link_tests;
mod tx_tests;
mod ulp_host_tests;
mod ulp_tests;

pub use helpers::{mac_text, xorshift};
