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

//! The things the included driver files take from `nonos_libc`. Grants are
//! numbered, and each release is recorded in order, per thread.

mod broker;
mod ipc;
mod mmio;
mod surface;
pub mod surface_registry;

pub use broker::PioGrantOut;
pub use broker::{calls, mk_device_claim, mk_device_release, mk_pio_grant, mk_pio_release};
pub use ipc::{mk_debug, mk_ipc_reply, replies, Deadline};
pub use mmio::{mk_mmio_map, mk_mmio_unmap, set_bar, MmioMapOut};
pub use surface::{
    fail_surface, mk_mmap, mk_munmap, mk_surface_register, surfaces, SurfaceDescriptor,
    SURFACE_FORMAT_ARGB8888,
};
pub use surface_registry::{fail_share, mk_surface_share, shares};
mod devices;
pub use devices::{
    mk_device_list, set_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI,
};
