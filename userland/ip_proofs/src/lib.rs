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

//! Host proofs for net.ip, run against the capsule's own source.
//!
//! Every module is included at the crate root where it names itself, except
//! main.rs, setup and the server's inbox loop. The proofs call the poll
//! handler the loop would have called; net.l2 answers behind the stand-in
//! nonos_libc.

extern crate alloc;

#[cfg(feature = "tcp-chaos")]
#[path = "../../capsule_net_ip/src/chaos.rs"]
pub mod chaos;
#[path = "../../capsule_net_ip/src/egress/mod.rs"]
pub mod egress;
#[path = "../../capsule_net_ip/src/icmp/mod.rs"]
pub mod icmp;
#[path = "../../capsule_net_ip/src/ingress.rs"]
pub mod ingress;
#[path = "../../capsule_net_ip/src/ipv4/mod.rs"]
pub mod ipv4;
#[path = "../../capsule_net_ip/src/l2_client/mod.rs"]
pub mod l2_client;
#[path = "../../capsule_net_ip/src/protocol/mod.rs"]
pub mod protocol;
// The capsule's own lint choices, allowed on the include rather than restyled.
#[allow(clippy::new_without_default, clippy::unnecessary_map_or)]
#[path = "../../capsule_net_ip/src/route/mod.rs"]
pub mod route;
pub mod server;
#[allow(clippy::new_without_default)]
#[path = "../../capsule_net_ip/src/state/mod.rs"]
pub mod state;

// The played link serves every proof file, and not every file uses each of
// its helpers.
#[cfg(test)]
#[allow(dead_code)]
mod link;
#[cfg(test)]
mod tests;
