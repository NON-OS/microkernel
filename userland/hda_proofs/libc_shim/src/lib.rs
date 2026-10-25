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

//! The things the included driver files take from `nonos_libc`.
//!
//! What matters most is the order. The broker frees a DMA frame the moment
//! the grant behind it goes, so every broker call is recorded in order, each
//! with what the controller's registers held as it was made, and a test reads
//! the teardown off that log: what was stopped before bus mastering went off,
//! and whether anything was given back before the release.

mod broker;
mod debug;
#[cfg(not(target_arch = "x86_64"))]
mod dma_sync;
mod grants;
mod pci;
mod pci_update;
mod records;
mod serve;
mod types;

pub use broker::{
    calls, mk_device_release, mk_dma_unmap, mk_irq_unbind, mk_mmio_unmap, watch, Call,
};
pub use debug::{logged, mk_debug};
#[cfg(not(target_arch = "x86_64"))]
pub use dma_sync::mk_dma_sync_for_cpu;
pub use grants::{mk_device_claim, mk_dma_map, mk_irq_bind, mk_mmio_map, offer_dma, offer_window};
pub use pci::{
    command, mk_pci_config_read, mk_pci_config_write, refuse_command, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE,
};
pub use pci_update::mk_pci_config_update;
pub use records::{mk_device_list, offer_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};
pub use serve::{mk_ipc_send, mk_irq_ack, mk_irq_wait, replies, MK_IRQ_WAIT_TIMED_OUT};
pub use types::{DmaMapOut, IrqBindOut, MmioMapOut, MK_IRQ_BIND_MSI, MK_IRQ_BIND_MSIX};

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
