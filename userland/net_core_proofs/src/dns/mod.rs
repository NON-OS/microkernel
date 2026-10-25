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

//! net.core's resolver: its errnos, its waiting callers, its verdicts, and
//! the gate that keeps a name in the clear to the Direct route.

#[path = "../../../capsule_net_core/src/server/handlers/dns/route_gate.rs"]
pub mod route_gate;

#[cfg(test)]
#[path = "../dns_errno_tests.rs"]
mod dns_errno_tests;
#[cfg(test)]
#[path = "../dns_pending_tests.rs"]
mod dns_pending_tests;
#[cfg(test)]
#[path = "../dns_verdict_tests.rs"]
mod dns_verdict_tests;
#[cfg(test)]
mod route_gate_tests;
