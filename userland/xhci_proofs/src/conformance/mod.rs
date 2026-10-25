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

//! The controller bring-up against a modelled host controller.

mod bringup_answer;
mod bringup_device;
mod bringup_model;
mod bringup_once;
mod bus_master_log;
mod bus_master_seen;
mod catch_up;
mod cnr_device;
pub(crate) mod devices;
mod erst_fetch;
mod left_running;
pub(crate) mod model;
pub(crate) mod port_device;
mod port_settle;

mod bus_master_assemble_tests;
mod bus_master_erst_tests;
mod bus_master_tests;
mod cap_tests;
mod debounce_failed_tests;
mod encoding_tests;
mod erst_fetch_tests;
mod halt_tests;
mod handoff_tests;
mod hub_context_tests;
mod interrupt_out_tests;
mod irq_bind_tests;
mod irq_park_tests;
mod park_tests;
mod payload_tests;
mod port_failed_tests;
mod port_tests;
mod reason_all;
mod reason_tests;
mod reset_tests;
mod revive_bus_master_tests;
mod revive_cnr_tests;
mod revive_tests;
mod route_tests;
mod setup_stage_tests;
mod silicon;
mod teardown_attempt;
mod teardown_tests;
mod window_tests;
