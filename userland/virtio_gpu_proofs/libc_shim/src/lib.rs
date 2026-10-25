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

//! What the included driver files take from `nonos_libc`, on the host.
//!
//! A reply is kept for the test to read rather than sent, a debug line is
//! kept rather than printed, a release or a PCI command write is kept and
//! its moment shown to the test rather than made, a DMA grant is host
//! memory kept per thread for a test to read back, and a device record
//! carries the three fields discovery matches on. `mk_yield` is what a wait
//! gives the processor up with; a test may hang a device step on it
//! (`on_yield`), so a model answers the ring on the driver's own thread, one
//! command at a time. There is no MkExit: an included file that
//! called it would not build.

mod debug;
mod dma;
mod held;
mod pci;
mod release;
mod reply;
mod yield_hook;

pub use debug::{mk_debug, said};
pub use dma::{mk_dma_map, DmaMapOut, MK_DMA_MAP_HIGH};
pub use held::{dma_reset, dma_spans, mk_dma_unmap};
pub use pci::{mk_pci_config_write, on_pci_write, pci_writes, MK_PCI_CFG_COMMAND};
pub use pci::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE};
pub use release::{mk_device_release, on_release, releases};
pub use reply::{mk_ipc_reply, replies};
pub use yield_hook::{mk_yield, off_yield, on_yield};

/// The fields of the broker's device record that discovery reads.
pub struct DeviceRecord {
    pub vendor: u16,
    pub device: u16,
    pub bus_kind: u8,
}

pub const BUS_KIND_PCI: u8 = 1;

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
