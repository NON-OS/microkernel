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

//! The network tests: the route, the routed reader, gathered flights, fetches and probes.

#[path = "fetch_tests.rs"]
mod fetch_tests;
#[path = "gather_end_tests.rs"]
mod gather_end_tests;
#[path = "gather_rig.rs"]
mod gather_rig;
#[path = "gather_sliced_tests.rs"]
mod gather_sliced_tests;
#[path = "gather_tests.rs"]
mod gather_tests;
#[path = "handshake_tests.rs"]
mod handshake_tests;
#[path = "network_tests.rs"]
mod network_tests;
#[path = "nox_format_tests.rs"]
mod nox_format_tests;
#[path = "private_route_gate_tests.rs"]
mod private_route_gate_tests;
#[path = "private_route_tests.rs"]
mod private_route_tests;
#[path = "probe_gate_tests.rs"]
mod probe_gate_tests;
#[path = "probe_model.rs"]
mod probe_model;
#[path = "route_host_tests.rs"]
mod route_host_tests;
#[path = "route_read_bounds_tests.rs"]
mod route_read_bounds_tests;
#[path = "route_read_tests.rs"]
mod route_read_tests;
#[path = "route_script.rs"]
mod route_script;
#[path = "route_tests.rs"]
mod route_tests;
#[path = "rpc_hostile_tests.rs"]
mod rpc_hostile_tests;
#[path = "tests.rs"]
mod tests;
#[path = "wallet_route_tests.rs"]
mod wallet_route_tests;
