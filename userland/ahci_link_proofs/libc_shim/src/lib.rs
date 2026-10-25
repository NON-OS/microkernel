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

//! What the open paths of the AHCI capsule and of the SD host capsule's eMMC
//! take from `nonos_libc`, on the host.
//!
//! Each broker call the driver makes is logged (`seen`) with the device as
//! the test's model reads it at that instant (`watch`), so a test holds the
//! driver to the order of `checks` and `released`: Bus Master on only once a
//! reset has been seen to finish and the engines read stopped, and the
//! release last, no DMA grant unmapped by hand before it. A grant is host
//! memory, no interrupt is ever raised, and any call can be refused.
//!
//! The register accessors and DMA barriers are the real libc's, mounted
//! so the driver files under test reach a register through the code they
//! ship with.

mod calls;
mod checks;
mod clock;
mod config;
mod console;
mod device;
mod dma;
mod dma_back;
#[path = "../../../libc/src/broker/dma_order.rs"]
mod dma_order;
mod grants;
mod log;
mod pci_bits;
#[path = "../../../libc/src/broker/mmio_access.rs"]
mod mmio_access;
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
pub use dma_order::{dma_mb, dma_rmb, dma_wmb};
pub use grants::{mk_irq_bind, mk_irq_unbind, mk_mmio_map, mk_mmio_unmap};
pub use log::{seen, watch};
pub use pci_bits::*;
pub use mmio_access::{
    mmio_read16, mmio_read32, mmio_read64, mmio_read8, mmio_write16, mmio_write32, mmio_write64,
    mmio_write8,
};
pub use refuse::{refuse, refuse_after};
pub use released::{bus_master_off, bus_master_off_after_stop, released_last};
pub use types::{Bar, DmaMapOut, IrqBindOut, MmioMapOut, BAR_KIND_MMIO, BAR_KIND_PIO};

/// The bring-up exit codes and give-up decision: the real nonos_libc
/// policy, at the path nonos_libc re-exports it under.
#[path = "../../../libc/src/bringup/policy.rs"]
pub mod bringup;
