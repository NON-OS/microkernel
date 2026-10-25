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

//! The server, request by request, in front of a modelled controller: the
//! slot table, the command ring and the controller's slots kept in step
//! through the commands that time out, and the hub operations.

mod abort_tests;
mod address_timeout_tests;
mod bench;
mod bench_driver;
mod bench_second;
mod bulk_reply_tests;
mod config_page_tests;
mod control_out_tests;
mod dead_buffer_tests;
mod dead_configure;
mod dead_configure_tests;
mod dead_stall_tests;
mod dead_tests;
mod disable_tests;
mod dispatch_fuzz_tests;
mod ep0_length_tests;
mod hc;
mod hc_bulk;
mod hc_bulk_run;
mod hc_command;
mod hc_device;
mod hc_ep0;
mod hc_execute;
mod hc_fill;
mod hc_input;
mod hc_race;
mod hc_recovery;
mod hc_reset;
mod hc_setup;
mod hub_bench;
mod hub_configure_refusal_tests;
mod hub_ep0_tests;
mod hub_refusal_tests;
mod hub_tests;
mod hub_tier_tests;
mod interrupt_payload_tests;
mod kept_bound_tests;
mod kept_named_tests;
mod mux_ghost_tests;
mod mux_refusal_tests;
mod mux_revive_tests;
mod mux_stale_slot_tests;
mod orphan_tests;
mod quarantine_tests;
mod served_tests;
mod shared_slot_tests;
mod stale_slot_tests;
mod stall_answer_tests;
mod status_tests;
mod stop_race_tests;
mod wire;
