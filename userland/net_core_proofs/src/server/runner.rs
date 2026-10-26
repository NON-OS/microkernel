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

//! The receive half of net.core's serve loop: how long it waits and what it
//! keeps of the request it took. The dispatch and the turn stay out.

#[path = "../../../capsule_net_core/src/server/runner/cadence.rs"]
mod cadence;
#[path = "../../../capsule_net_core/src/server/runner/receive.rs"]
pub mod receive;
#[cfg(test)]
#[path = "receive_tests.rs"]
mod receive_tests;
