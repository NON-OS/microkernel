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

//! The things the included BGA driver takes from `nonos_libc`: the device
//! list discovery reads, the claim, the PCI command register, the two BAR
//! mappings, the surface's memory, its registration, each share of it and its
//! release, the service and its inbox, and the driver's pid. A test says what
//! each BAR maps to and how long the broker says the mapping is, so a mapping
//! shorter than the memory behind it shows any write past its end, and a
//! register mapping given back holds poison until the release (`seal`). A
//! test's adapter model answers each register write as it is made (`hook`).
//! The receive policy is the libc's own (`bringup`).

pub mod bringup;
mod claim;
mod hook;
mod idle;
mod ipc;
mod mem;
mod mmio;
mod pci;
mod records;
mod seal;
mod service;
mod surface;
pub mod surface_registry;
mod watch;

pub use bringup::run::recv_ready;
pub use claim::{given_back, mk_device_claim, mk_device_release, registers_at_give_back};
pub use hook::{on_w16, wrote16};
pub use idle::{idles, mk_debug, mk_idle_ms, mk_uptime_ms};
pub use ipc::{mk_ipc_recv_from, mk_ipc_reply, replies, send, waits, ERRNO_TIMEDOUT};
pub use mem::{lend, mk_mmap, mk_munmap};
pub use mmio::{map_bar, mk_mmio_map, mk_mmio_unmap, MmioMapOut, MK_MMIO_MAP_WC};
pub use pci::{
    command, mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_MEMORY_SPACE, PCI_CMD_IO_SPACE,
};
pub use records::{mk_device_list, offer, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};
pub use seal::untouched_since_unmap;
pub use service::{hold, mk_getpid, mk_service_lookup, mk_service_register, services};
pub use surface::{
    mk_surface_register, refuse_surfaces, registered, SurfaceDescriptor, SHARED_HANDLE_BASE,
    SURFACE_FORMAT_ARGB8888,
};
pub use surface_registry::{mk_surface_release, mk_surface_share, released, shares, Share};
pub use watch::watch;
