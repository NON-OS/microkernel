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

//! The driver against modelled capability regions, a part that consumes the
//! control queue, and a host model on the display wire; and the orders in
//! its source that only a boot runs.

mod gpu;
mod harness;
mod host;
mod model;
mod part;
mod predecessor;
mod probe_stall;
mod wire;

mod clock_tests;
mod config_tests;
mod discovery_tests;
mod late_tests;
mod master_tests;
mod mode_tests;
mod modern_queue_tests;
mod modern_refusal_tests;
mod modern_tests;
mod notify_bound_tests;
mod order_tests;
mod probe_stall_tests;
mod queue_limit_tests;
mod queue_tests;
mod recover_order_tests;
mod reply_refusal_tests;
mod reply_sweep;
mod reply_tests;
mod teardown_tests;
mod wire_scanout_tests;
mod wire_sweep;
mod wire_tests;
