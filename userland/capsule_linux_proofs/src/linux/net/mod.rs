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

//! The counts and sets poll and select take, the resolver, and the path a
//! stream outside the family takes: the route decision, what is said to
//! net.anon and read back, and the socket table's bookkeeping for it.

#[path = "../../../../capsule_linux/src/linux/net/ready_sets.rs"]
pub mod ready_sets;

pub mod dns;

/* The route every capsule holding Network leaves by: nonos_route_link's own. */
pub use crate::pick::{Route, ANYONE_DOWN};

#[path = "../../../../capsule_linux/src/linux/net/guest_route.rs"]
pub mod guest_route;

#[path = "../../../../capsule_linux/src/linux/net/anon_answer.rs"]
pub mod anon_answer;
#[path = "../../../../capsule_linux/src/linux/net/anon_ops.rs"]
pub mod anon_ops;
#[path = "../../../../capsule_linux/src/linux/net/anon_wire.rs"]
pub mod anon_wire;

pub mod sock;

/* The two calls the table makes to close a stream, recorded, not sent. */
pub mod anon_stream;
pub mod stream;
