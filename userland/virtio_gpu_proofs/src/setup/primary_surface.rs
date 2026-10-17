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
//! The primary surface's files, each named, since the driver declares
//! them as children of a file this crate mounts from elsewhere.

#[path = "../../../capsule_driver_virtio_gpu/src/setup/primary_surface/create.rs"]
mod create;
#[path = "../../../capsule_driver_virtio_gpu/src/setup/primary_surface/descriptor.rs"]
mod descriptor;
#[path = "../../../capsule_driver_virtio_gpu/src/setup/primary_surface/dma.rs"]
pub mod dma;
#[path = "../../../capsule_driver_virtio_gpu/src/setup/primary_surface/geometry.rs"]
mod geometry;
#[path = "../../../capsule_driver_virtio_gpu/src/setup/primary_surface/prime.rs"]
mod prime;
#[path = "../../../capsule_driver_virtio_gpu/src/setup/primary_surface/resource.rs"]
mod resource;
#[path = "../../../capsule_driver_virtio_gpu/src/setup/primary_surface/state.rs"]
mod state;

pub use self::create::create;
pub use self::state::Primary;
