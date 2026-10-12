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

//! The primary surface's record, geometry, table entry and the commands
//! that show it, and the mode a scanout is taken at, as they ship. The rest
//! of setup calls the broker.

#[path = "../../capsule_driver_virtio_gpu/src/setup/primary_surface/geometry.rs"]
pub mod geometry;
#[path = "../../capsule_driver_virtio_gpu/src/setup/mode.rs"]
pub mod mode;
#[path = "../../capsule_driver_virtio_gpu/src/setup/primary_surface/prime.rs"]
pub mod prime;
#[path = "../../capsule_driver_virtio_gpu/src/setup/primary_surface/resource.rs"]
pub mod resource;
#[path = "../../capsule_driver_virtio_gpu/src/setup/primary_surface/state.rs"]
mod state;

pub use state::Primary;
