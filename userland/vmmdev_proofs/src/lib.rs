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

//! Host proofs for the VirtualBox guest device driver. Every driver file
//! below is the shipping source, included by path; only the broker under
//! them is the shim's, the request port answered by a model of VMMDev.

extern crate alloc;

#[path = "../../capsule_driver_vmmdev/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_vmmdev/src/discover.rs"]
pub mod discover;
pub mod model;
#[path = "../../capsule_driver_vmmdev/src/post.rs"]
pub mod post;
#[path = "../../capsule_driver_vmmdev/src/regs/mod.rs"]
pub mod regs;
#[path = "../../capsule_driver_vmmdev/src/req/mod.rs"]
pub mod req;
#[path = "../../capsule_driver_vmmdev/src/say.rs"]
pub mod say;
#[path = "../../capsule_driver_vmmdev/src/serve.rs"]
pub mod serve;
pub mod setup;

#[cfg(test)]
mod tests;
