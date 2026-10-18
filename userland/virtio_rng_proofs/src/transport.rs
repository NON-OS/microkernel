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

//! The transport as it ships, minus the kernel calls: the doorbell and the
//! reset, over a register window in memory.

#[path = "../../capsule_driver_virtio_rng/src/transport/access.rs"]
mod access;
#[path = "../../capsule_driver_virtio_rng/src/transport/reset.rs"]
mod reset;
#[path = "../../capsule_driver_virtio_rng/src/transport/types.rs"]
mod types;

pub use types::{Modern, Transport};
