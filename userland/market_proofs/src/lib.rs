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

//! The market capsule's readiness gate, included and exercised.

extern crate alloc;

pub mod bonds;
pub mod install_ready;

#[path = "../../capsule_market/src/server/handlers/install_ready/find_release.rs"]
pub mod find_release;

#[cfg(test)]
mod abi_v3;
// The request header decode every frame passes before dispatch (protocol/).
#[cfg(test)]
mod protocol;
mod server;
#[cfg(test)]
use abi_v3::chain_hosts;

#[cfg(test)]
mod index_decode_tests;

pub mod release_wire;

pub mod meter;
// The meter's files name each other from its crate root.
use meter::{budget, channel, error, state};
pub mod store;

pub mod linux_why;
#[cfg(test)]
mod proto_hostile;
#[cfg(test)]
mod reason_tests;

#[cfg(test)]
mod readiness_tests;
pub mod terminal;

#[cfg(test)]
mod release_tests;
#[cfg(test)]
mod request_tests;

#[cfg(test)]
mod store_checks;

#[cfg(test)]
mod gate_fit_tests;
#[cfg(test)]
mod readiness_wire_tests;
#[cfg(test)]
mod release_escrow_tests;
#[cfg(test)]
mod wire_tests;
