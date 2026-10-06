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

//! Host proofs for net.tcp, run against the capsule's own source.
//!
//! Every module of the capsule is included at the crate root where it names
//! itself, except main.rs (the entry point) and the server's inbox loop. The
//! proofs call the request handlers the loop would have called, and a peer
//! played behind the stand-in nonos_libc answers the segments the capsule
//! sends and feeds it the ones a network would.

extern crate alloc;

#[path = "../../capsule_net_tcp/src/clock.rs"]
pub mod clock;
#[path = "../../capsule_net_tcp/src/ip_client/mod.rs"]
pub mod ip_client;
#[path = "../../capsule_net_tcp/src/protocol/mod.rs"]
pub mod protocol;
pub mod server;
// The capsule's own lint choices, allowed on the include rather than restyled.
#[allow(clippy::new_without_default, clippy::while_let_loop)]
#[path = "../../capsule_net_tcp/src/state/mod.rs"]
pub mod state;
#[allow(clippy::new_without_default)]
#[path = "../../capsule_net_tcp/src/tcp/mod.rs"]
pub mod tcp;

// The played peer serves every proof file, and not every file uses each of
// its helpers.
#[cfg(test)]
#[allow(dead_code)]
mod peer;
#[cfg(test)]
mod tests;
