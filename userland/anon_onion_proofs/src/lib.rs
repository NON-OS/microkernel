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


//! net.anon's onion service client, end to end on the host.
//!
//! The capsule's own modules are compiled here by path, unchanged. Only the
//! link to the guard is replaced (`link.rs`, a pair of queues) and nonos_libc
//! (`shim/`). The tests drive the capsule's ticks and play every relay and
//! the onion service themselves, from the fork's rules and with crypto
//! crates of their own, so the client is checked against an implementation
//! that is not itself.

#![no_std]
/*
 * The capsule's tree is compiled here whole, and only the onion path is
 * driven, so much of it is unused in this crate. The two clippy lints are
 * findings in capsule files this crate includes and does not own (the
 * cell Frame's variant sizes, circuit_tick's map_or); they are allowed here
 * rather than changed from a proofs crate.
 */
#![allow(dead_code, unused_imports, clippy::large_enum_variant, clippy::unnecessary_map_or)]

extern crate alloc;
#[cfg(test)]
extern crate std;

#[path = "../../capsule_net_anon/src/base64_encode.rs"]
mod base64_encode;
#[path = "../../capsule_net_anon/src/cell/mod.rs"]
pub mod cell;
#[path = "../../capsule_net_anon/src/circuit/mod.rs"]
pub mod circuit;
#[path = "../../capsule_net_anon/src/crypto/mod.rs"]
pub mod crypto;
#[path = "../../capsule_net_anon/src/directory/mod.rs"]
pub mod directory;
pub mod link;
#[path = "../../capsule_net_anon/src/manager/mod.rs"]
pub mod manager;
#[path = "../../capsule_net_anon/src/ntor/mod.rs"]
pub mod ntor;
#[path = "../../capsule_net_anon/src/onion/mod.rs"]
pub mod onion;
#[path = "../../capsule_net_anon/src/path/mod.rs"]
pub mod path;
#[path = "../../capsule_net_anon/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_net_anon/src/stream/mod.rs"]
pub mod stream;
#[path = "../../capsule_net_anon/src/tcp_client/mod.rs"]
mod tcp_client;
#[path = "../../capsule_net_anon/src/trace/mod.rs"]
mod trace;

#[cfg(test)]
mod tests;
