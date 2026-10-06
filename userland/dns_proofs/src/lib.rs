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

//! Host proofs for net.dns, run against the capsule's own source.
//!
//! Every module the resolve path uses is included at the crate root where it
//! names itself; main.rs, setup and the inbox loop are left out. The proofs
//! call the resolve handlers the loop would have called, and net.udp and the
//! upstream resolver answer behind the stand-in nonos_libc.

extern crate alloc;

// The capsule's own lint choices, allowed on the include rather than restyled.
#[allow(clippy::new_without_default, clippy::unnecessary_map_or)]
#[path = "../../capsule_net_dns/src/dns/mod.rs"]
pub mod dns;
#[path = "../../capsule_net_dns/src/protocol/mod.rs"]
pub mod protocol;
pub mod server;
#[path = "../../capsule_net_dns/src/state.rs"]
pub mod state;
#[path = "../../capsule_net_dns/src/udp_client/mod.rs"]
pub mod udp_client;

// The played network serves every proof file, and not every file uses each
// of its helpers.
#[cfg(test)]
#[allow(dead_code)]
mod upstream;
#[cfg(test)]
mod tests;
