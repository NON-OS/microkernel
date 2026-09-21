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

//! The link certificate chain, included from the real capsule source.

#[path = "../../../capsule_net_anon/src/link/certs.rs"]
pub mod certs;

#[path = "../../../capsule_net_anon/src/link/constants.rs"]
pub mod constants;

#[path = "../../../capsule_net_anon/src/link/ed_cert/mod.rs"]
pub mod ed_cert;

#[path = "../../../capsule_net_anon/src/link/bind/mod.rs"]
pub mod bind;

#[path = "../../../capsule_net_anon/src/link/versions.rs"]
pub mod versions;

pub use bind::bind;
pub use bind::error::BindError;
