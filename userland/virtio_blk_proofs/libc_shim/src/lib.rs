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
//! The interrupt line and the clock are one model: a wait on the line takes
//! the time the test's script says, and the clock moves by that much, so a
//! wait bounded by the clock and one bounded by a count of wakes come apart
//! here as they would on a machine. The broker grants what a bring-up asks
//! and logs each request with a hook the test runs at that moment; port
//! I/O reaches a device model the test attaches, which reaches the DMA
//! grants by their device addresses. The record types and the bring-up
//! schedule are the libc's own, mounted from its source; MkExit unwinds.

#[path = "../../../libc/src/bringup/mod.rs"]
pub mod bringup;
mod broker;
mod clock;
mod consts;
mod debug;
mod device;
mod dma;
mod events;
mod exit;
mod grants;
mod irq;
mod line;
mod memory;
mod pio;
pub mod types;

pub use bringup::{bring_up, EXIT_GAVE_UP};
pub use broker::present_config;
pub use broker::{mk_device_claim, mk_device_release, mk_pci_config_read, mk_pci_config_write};
pub use clock::{advance, break_clock, mk_idle_ms, mk_uptime_ms, now};
pub use consts::*;
pub use debug::{mk_debug, said};
pub use device::{mk_device_list, present};
pub use dma::mk_dma_map;
pub use events::{events, on_event, Event};
pub use exit::mk_exit;
pub use grants::{mk_irq_bind, mk_mmio_map, mk_mmio_unmap, mk_pio_grant, offer_irq};
pub use irq::{mk_irq_ack, mk_irq_poll, mk_irq_wait, MK_IRQ_WAIT_TIMED_OUT};
pub use line::{acks, on_wait, waits, Wake};
pub use memory::{dma_read, dma_write};
pub use pio::{attach_ports, mk_pio_read, mk_pio_write, Ports};
pub use types::bar::{Bar, BAR_KIND_MMIO, BAR_KIND_NONE, BAR_KIND_PIO};
pub use types::bus::BUS_KIND_PCI;
pub use types::device::DeviceRecord;
pub use types::dma::DmaMapOut;
pub use types::irq::{IrqBindOut, IrqPollOut};
pub use types::mmio::MmioMapOut;
pub use types::pio::PioGrantOut;

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
