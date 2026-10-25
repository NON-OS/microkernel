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

//! Host proofs for the netvsc driver. The driver's files are the shipping
//! source, included by path; only the libc under them is the shim's, and
//! the host is a model.

extern crate alloc;

#[path = "../../capsule_driver_netvsc/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_netvsc/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_netvsc/src/link.rs"]
pub mod link;
#[path = "../../capsule_driver_netvsc/src/nvsp.rs"]
pub mod nvsp;
#[path = "../../capsule_driver_netvsc/src/probe.rs"]
pub mod probe;
#[path = "../../capsule_driver_netvsc/src/rndis.rs"]
pub mod rndis;
#[path = "../../capsule_driver_netvsc/src/start.rs"]
pub mod start;
#[path = "../../capsule_driver_netvsc/src/station.rs"]
pub mod station;

pub mod model;

#[cfg(test)]
mod tests;
