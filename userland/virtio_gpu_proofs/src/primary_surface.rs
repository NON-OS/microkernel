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

//! Bring-up's primary surface as it ships: create.rs beside the geometry,
//! commands and table entry setup.rs already mounts, as its own directory
//! lays them out. It registers the surface and shares it with no one (K20).

#[path = "../../capsule_driver_virtio_gpu/src/setup/primary_surface/create.rs"]
pub mod create;
#[path = "../../capsule_driver_virtio_gpu/src/setup/primary_surface/descriptor.rs"]
mod descriptor;
#[path = "../../capsule_driver_virtio_gpu/src/setup/primary_surface/dma.rs"]
mod dma;

use crate::setup::{geometry, prime, resource};

mod state {
    pub use crate::setup::Primary;
}
