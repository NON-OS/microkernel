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

//! Host proofs for the VMBus core. Every driver file below is the shipping
//! source, included by path; only the libc under them is the shim's, the
//! SynIC answered by a model of a VMBus host behind the kernel's own check
//! of what the core may post.

#[path = "../../capsule_driver_vmbus/src/connect.rs"]
pub mod connect;
#[path = "../../capsule_driver_vmbus/src/constants.rs"]
pub mod constants;
#[path = "../../capsule_driver_vmbus/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_vmbus/src/give.rs"]
pub mod give;
#[path = "../../capsule_driver_vmbus/src/guid.rs"]
pub mod guid;
#[path = "../../capsule_driver_vmbus/src/hv.rs"]
pub mod hv;
pub mod model;
#[path = "../../capsule_driver_vmbus/src/msg.rs"]
pub mod msg;
#[path = "../../capsule_driver_vmbus/src/offers.rs"]
pub mod offers;
#[path = "../../capsule_driver_vmbus/src/say.rs"]
pub mod say;
#[path = "../../capsule_driver_vmbus/src/serve.rs"]
pub mod serve;
#[path = "../../capsule_driver_vmbus/src/setup/mod.rs"]
pub mod setup;
#[path = "../../capsule_driver_vmbus/src/wait.rs"]
pub mod wait;

#[cfg(test)]
mod tests;
