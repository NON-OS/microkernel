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

//! The things the included driver files take from `nonos_libc`, each backed
//! by something a test can set or read: the host clock, a switchable entropy
//! source, a per-thread console, the broker's device list and BAR map, a
//! scripted config space, the PCI Command register, a record of every grant
//! given back, and the IPC calls the server loop makes.

mod clock;
mod command;
mod command_write;
mod debug;
mod devices;
mod entropy;
mod ipc;
mod mmio;
mod pci;
mod release;
mod steps;

pub use clock::{mk_idle_ms, Deadline};
pub use command::{command, refuse_bus_master, set_command};
pub use command::{MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};
pub use command_write::{mk_pci_config_update, mk_pci_config_write};
pub use debug::{clear_log, logged, mk_debug};
pub use devices::{mk_device_list, present, Bar, DeviceRecord};
pub use devices::{BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
pub use entropy::{crypto_random, entropy, Entropy};
pub use ipc::{line_up_requests, mk_ipc_recv_from, mk_ipc_reply, recv_ready, replies, SERVED};
pub use mmio::{map_at_most, mk_mmio_map, MmioMapOut};
pub use pci::{config_reads, mk_pci_config_read, set_ring_status};
pub use release::{given_back, mk_device_release, mk_dma_unmap, mk_mmio_unmap};
pub use steps::{seen, watch, Part, Seen, Step};

// The real libc's register accessors and DMA barriers, mounted so the driver
// files under test reach a device through the code they ship with.
#[path = "../../../libc/src/broker/dma_order.rs"]
mod dma_order;
#[path = "../../../libc/src/broker/mmio_access.rs"]
mod mmio_access;
pub use dma_order::{dma_mb, dma_rmb, dma_wmb};
pub use mmio_access::{
    mmio_read16, mmio_read32, mmio_read64, mmio_read8, mmio_write16, mmio_write32, mmio_write64,
    mmio_write8,
};
