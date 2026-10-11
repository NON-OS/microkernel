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

//! Host proofs for the virtio-input driver. The driver files are the
//! shipping source, included by path; only the keyboard's layout source,
//! which asks the policy store, is replaced by a per-thread setting.

#[path = "../../capsule_driver_virtio_input/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_input/src/events/mod.rs"]
pub mod events;
pub mod init;
pub mod keyboard;
#[path = "../../capsule_driver_virtio_input/src/post.rs"]
pub mod post;
pub mod ps2;
#[path = "../../capsule_driver_virtio_input/src/queue/mod.rs"]
pub mod queue;

#[cfg(test)]
mod tests;
