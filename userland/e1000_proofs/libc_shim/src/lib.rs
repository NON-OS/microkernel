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

//! The things the included driver files take from `nonos_libc`, and the
//! clock hook a test's part acts on the driver's looks through.
//!
//! `crypto_random` is the one that matters. The station address is drawn
//! from it, and the driver's promise is that when there is no entropy the
//! card is never brought on the air under the address in its EEPROM. The
//! tests switch it off to hold the driver to that.

mod deadline;
mod debug;
mod devices;
mod entropy;
mod grants;
mod ipc;
mod look;
mod mmio;
mod pci;
mod pci_write;
mod steps;

pub use deadline::{uptime_ms, Deadline};
pub use debug::{logged, mk_debug};
pub use devices::{mk_device_list, present, Bar, DeviceRecord};
pub use devices::{BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
pub use entropy::{crypto_random, entropy, Entropy};
pub use grants::{given_back, mk_device_release, mk_dma_unmap, mk_irq_unbind, mk_mmio_unmap};
pub use ipc::{line_up_requests, mk_ipc_recv_from, mk_ipc_reply, recv_ready, replies, SERVED};
pub use look::{on_clock, OnClock};
pub use mmio::{map_at_most, mk_mmio_map, MmioMapOut};
pub use pci::{command, refuse_bus_master, set_command};
pub use pci::{MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};
pub use pci_write::{mk_pci_config_update, mk_pci_config_write};
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
