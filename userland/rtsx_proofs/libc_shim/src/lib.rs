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

//! What the included rtsx driver files take from `nonos_libc`, on the host
//! and per test thread: a virtual clock, the console, the broker's claim,
//! maps and release, DMA memory a modelled reader reaches by its device
//! address while bus mastering is on, and the IPC the block service answers
//! on. Every broker call and every event a model notes lands in one ordered
//! list, so a test can hold a teardown to its order.

mod caps;
mod claim;
mod clock;
mod command;
mod console;
mod devices;
mod dma;
mod events;
mod grants;
mod hook;
mod ipc;
mod mmio;
mod reach;
mod replies;
mod tally;
mod types;

pub use caps::{grant_cap, mk_cap_check};
pub use claim::{bus_master, mk_device_claim, mk_device_release, EPOCH};
pub use clock::{advance_ms, mk_idle_ms, mk_uptime_ms, now_ms, sleeps, uptime_reads};
pub use command::{mk_pci_config_write, refuse_bus_master, refuse_command};
pub use console::{mk_debug, said};
pub use devices::{mk_device_list, set_devices};
pub use dma::{mk_dma_map, mk_dma_unmap, refuse_dma_map};
pub use events::{events, note};
pub use grants::live_grants;
pub use hook::{device_acts, set_device_hook};
pub use ipc::{mk_ipc_recv_from, push_request, recv_ready};
pub use mmio::{mk_mmio_map, mk_mmio_unmap, MMIO_VA};
pub use reach::{dma_read, dma_write};
pub use replies::{mk_ipc_reply, mk_ipc_send, take_sent, To};
pub use tally::{blocked, violations};
pub use types::*;

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
