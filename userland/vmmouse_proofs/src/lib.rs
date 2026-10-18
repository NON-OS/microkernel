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

//! Host proofs for the VMware absolute pointer driver. Every driver file
//! below is the shipping source, included by path; only the libc under
//! them is the shim's, the backdoor answered by a model of QEMU's vmmouse
//! behind the kernel's own allowlist.

#[path = "../../capsule_driver_vmmouse/src/backdoor.rs"]
pub mod backdoor;
#[path = "../../capsule_driver_vmmouse/src/constants.rs"]
pub mod constants;
pub mod model;
#[path = "../../capsule_driver_vmmouse/src/packet.rs"]
pub mod packet;
#[path = "../../capsule_driver_vmmouse/src/post.rs"]
pub mod post;
#[path = "../../capsule_driver_vmmouse/src/say.rs"]
pub mod say;
#[path = "../../capsule_driver_vmmouse/src/serve.rs"]
pub mod serve;
#[path = "../../capsule_driver_vmmouse/src/setup.rs"]
pub mod setup;

#[cfg(test)]
mod tests;
