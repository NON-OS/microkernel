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

//! Host proofs for net.sockets' socket table, run on the real source.

extern crate alloc;

#[allow(clippy::manual_flatten)]
#[path = "../../capsule_net_sockets/src/sockets/mod.rs"]
pub mod sockets;

/// The table's two bounds, read from the same source the capsule builds.
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "../../capsule_net_sockets/src/sockets/table/types.rs"]
mod bounds;

// The wire's errnos, magic and ops, which the header decode names.
#[path = "../../capsule_net_sockets/src/protocol/mod.rs"]
pub mod protocol;

// The header decode and the refusal reply.
pub mod server;

#[cfg(test)]
mod refusal_tests;
#[cfg(test)]
mod table_tests;
