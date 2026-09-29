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

//! The directory parser, included from the real capsule source.

#[path = "../../../capsule_net_anon/src/directory/base64.rs"]
pub mod base64;

#[path = "../../../capsule_net_anon/src/directory/lines.rs"]
pub mod lines;

#[path = "../../../capsule_net_anon/src/directory/number.rs"]
pub mod number;

#[path = "../../../capsule_net_anon/src/directory/time.rs"]
pub mod time;

#[path = "../../../capsule_net_anon/src/directory/consensus/mod.rs"]
pub mod consensus;

#[path = "../../../capsule_net_anon/src/directory/microdesc/mod.rs"]
pub mod microdesc;

pub use base64::decode;

#[path = "../../../capsule_net_anon/src/base64_encode.rs"]
pub mod base64_encode;

pub use base64_encode::encode;
pub use lines::{arg, args, lines};
pub use number::{decimal, ipv4, port};

/*
 * The certificate parser, with the span helper it reaches for as `super::span`.
 * Both sit here rather than under a nested module so that reference resolves the
 * way it does in the capsule. Anchoring itself wants an RSA verify that lives
 * behind an IPC call to the crypto capsule, so it has no host equivalent and is
 * not pulled in.
 */

#[path = "../../../capsule_net_anon/src/directory/verify/span.rs"]
pub mod span;

#[path = "../../../capsule_net_anon/src/directory/verify/cert.rs"]
pub mod cert;

pub mod verify {
    pub use super::cert::parse;
}

#[path = "../../../capsule_net_anon/src/directory/authority/mod.rs"]
pub mod authority;

#[path = "directory/fetch.rs"]
pub mod fetch;
