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

//! net.ip's gate: an ICMP message leaves in the clear only on the Direct
//! route, and only from a send body that holds a whole ICMP header.

#[path = "../../../capsule_net_core/src/server/handlers/ip/route_gate.rs"]
pub mod route_gate;

#[cfg(test)]
mod route_gate_tests;

#[path = "../../../capsule_net_core/src/server/handlers/ip/send_body.rs"]
pub mod send_body;

#[cfg(test)]
mod send_body_tests;
