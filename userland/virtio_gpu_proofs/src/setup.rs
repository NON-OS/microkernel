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

//! The transport (the queue's grant and the PCI command from the shim), the
//! scanouts read and the mode each is taken at, and the best-effort probes
//! (output 0's EDID and the 3D path), as they ship. The rest of setup calls
//! the broker or the kernel's surface pool.

#[path = "../../capsule_driver_virtio_gpu/src/setup/dma.rs"]
mod dma;
#[path = "../../capsule_driver_virtio_gpu/src/setup/edid.rs"]
pub mod edid;
#[path = "../../capsule_driver_virtio_gpu/src/setup/mode.rs"]
pub mod mode;
#[path = "../../capsule_driver_virtio_gpu/src/setup/pci.rs"]
pub mod pci;
#[path = "../../capsule_driver_virtio_gpu/src/setup/probe_3d/mod.rs"]
mod probe_3d;
#[path = "../../capsule_driver_virtio_gpu/src/setup/probes.rs"]
pub mod probes;
#[path = "../../capsule_driver_virtio_gpu/src/setup/scanouts.rs"]
pub mod scanouts;
#[path = "../../capsule_driver_virtio_gpu/src/setup/transport.rs"]
pub mod transport;
