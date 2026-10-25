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

mod boot_framebuffer;
mod device;
// The kernel's one definition of DGON, the device-gone notice, by path.
#[path = "../../../../src/hardware/broker/acpi/dgon_wire.rs"]
mod dgon;
mod dma;
mod dma_lines;
mod dma_order;
// dma_sync.rs is x86_64's; each other architecture has its own file.
#[cfg_attr(target_arch = "aarch64", path = "dma_sync_aarch64.rs")]
#[cfg_attr(target_arch = "riscv64", path = "dma_sync_riscv64.rs")]
mod dma_sync;
// The hypervisor calls, one file per host interface (`hv`).
mod hv;
mod irq;
mod irq_wait;
mod mmio;
mod mmio_access;
mod pci;
mod pci_update;
mod pio;
mod types;

pub use boot_framebuffer::{mk_boot_framebuffer, BootFramebufferInfo, FIRMWARE_BOOT_FRAMEBUFFER};
pub use device::{mk_device_claim, mk_device_list, mk_device_release};
pub use dgon::{dgon_encode, dgon_parse, Dgon, DGON_LEN, DGON_TAG, DGON_VERSION, WHY_GONE};
pub use dma::{
    mk_dma_map, mk_dma_unmap, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32, MK_DMA_MAP_FROM_DEVICE,
    MK_DMA_MAP_HIGH, MK_DMA_MAP_TO_DEVICE, MK_DMA_MAP_WC,
};
pub use dma_lines::{ctr_dminline_bytes, dma_sync_lines, dma_sync_lines_of, DMA_SYNC_LINE};
pub use dma_order::{dma_mb, dma_rmb, dma_wmb};
pub use dma_sync::{mk_dma_sync_for_cpu, mk_dma_sync_for_device};
pub use hv::*;
pub use irq::{
    mk_irq_ack, mk_irq_bind, mk_irq_poll, mk_irq_unbind, MK_IRQ_BIND_MSI, MK_IRQ_BIND_MSIX,
};
pub use irq_wait::{mk_irq_wait, MK_IRQ_WAIT_TIMED_OUT};
pub use mmio::{mk_mmio_map, mk_mmio_unmap, MK_MMIO_MAP_WC};
pub use mmio_access::{
    mmio_read16, mmio_read32, mmio_read64, mmio_read8, mmio_write16, mmio_write32, mmio_write64,
    mmio_write8,
};
pub use pci::{
    mk_pci_config_read, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE, MK_PCI_MSIX_CTRL_ENABLE,
    MK_PCI_MSIX_CTRL_FUNCTION_MASK,
};
pub use pci_update::{mk_pci_config_update, MK_PCI_CFG_MASK_SHIFT};
pub use pio::{mk_pio_grant, mk_pio_read, mk_pio_release, mk_pio_write};
pub use types::{
    Bar, DeviceRecord, DmaMapOut, IrqBindOut, IrqPollOut, MmioMapOut, PioGrantOut, BAR_FLAG_FIXED,
    BAR_FLAG_MEM64, BAR_FLAG_PREFETCH, BAR_KIND_MMIO, BAR_KIND_NONE, BAR_KIND_PIO, BUS_KIND_ACPI,
    BUS_KIND_FDT, BUS_KIND_FIRMWARE, BUS_KIND_PCI, BUS_KIND_VIRT, DEVICE_FLAG_CLAIMED,
    DEVICE_FLAG_DEPS_UNMET, DEVICE_FLAG_DISABLED, DEVICE_FLAG_DMA_NONCOHERENT,
    DEVICE_FLAG_EXTERNAL, DEVICE_FLAG_IRQ_ACTIVE_LOW, DEVICE_FLAG_IRQ_EDGE, FDT_AHCI, FDT_SDHCI,
    FDT_XHCI, NUMA_UNKNOWN,
};
