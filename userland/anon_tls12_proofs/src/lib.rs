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


//! net.anon's TLS 1.2 link client, compiled from the capsule's own files.
//!
//! Every `#[path]` module is the file the capsule builds. The shims give
//! them the crate paths they expect: `nonos_libc` answered by RustCrypto with
//! seeded randomness, `nonos_tls` reduced to its Io trait and an RSA verify
//! from the rsa crate.

extern crate alloc;

#[path = "shim/crypto.rs"]
pub mod crypto;
#[path = "shim/trace.rs"]
pub mod trace;

#[path = "../../capsule_net_anon/src/link/tls12/mod.rs"]
pub mod tls12;

#[cfg(test)]
mod tests;

#[path = "../../capsule_net_anon/src/link/fallback.rs"]
pub mod fallback;
