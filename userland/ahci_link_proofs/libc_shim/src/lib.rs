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

//! What the AHCI capsule's open paths, SATA and eMMC, take from
//! `nonos_libc`, on the host.
//!
//! Each broker call the driver makes is logged (`seen`) with the device as
//! the test's model reads it at that instant (`watch`), so a test holds the
//! driver to the order of `checks` and `released`: Bus Master on only once a
//! reset has been seen to finish and the engines read stopped, and the
//! release last, no DMA grant unmapped by hand before it. A grant is host
//! memory, no interrupt is ever raised, and any call can be refused.

mod calls;
mod checks;
mod clock;
mod config;
mod console;
mod device;
mod dma;
mod dma_back;
mod grants;
mod log;
mod refuse;
mod released;
mod types;

pub use calls::{Call, Engines, Seen};
pub use checks::{bus_master_after_reset, first_bus_master, refused_without_bus_master};
pub use clock::{mk_uptime_ms, uptime};
pub use config::{config_reads, set_config};
pub use console::{mk_debug, said};
pub use device::{mk_device_claim, mk_device_release, mk_pci_config_read, mk_pci_config_write};
pub use dma::mk_dma_map;
pub use dma_back::{live_dma, mk_dma_unmap};
pub use grants::{mk_irq_bind, mk_irq_unbind, mk_mmio_map, mk_mmio_unmap};
pub use log::{seen, watch};
pub use refuse::{refuse, refuse_after};
pub use released::{bus_master_off, bus_master_off_after_stop, released_last};
pub use types::{Bar, DmaMapOut, IrqBindOut, MmioMapOut, BAR_KIND_MMIO, BAR_KIND_PIO};

/// The bring-up exit codes and give-up decision: the real nonos_libc
/// policy, at the path nonos_libc re-exports it under.
#[path = "../../../libc/src/bringup/policy.rs"]
pub mod bringup;

/// The PCI Command bits and the DMA flag, as nonos_libc defines them.
pub const MK_PCI_CFG_COMMAND: u32 = 0x04;
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
pub const MK_PCI_CMD_INTX_DISABLE: u16 = 1 << 10;
pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;
