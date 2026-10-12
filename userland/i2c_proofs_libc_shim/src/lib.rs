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

//! What the included i2c_pci and i2c_hid driver files take from `nonos_libc`,
//! answered on the calling thread.
//!
//! The controller side replies through `mk_ipc_reply`, which lands in a
//! mailbox the test drains. The HID side calls through `mk_ipc_call_timeout`,
//! which hands the request to whatever service the test registered and
//! returns that service's reply, so the two drivers' real wire code meets in
//! one process with no kernel between them.

mod call;
mod input;
mod misc;
mod raw;
mod reply;
mod time;

pub use call::{mk_ipc_call_timeout, mk_service_lookup, serve, Served, SERVICE_PID, SERVICE_PORT};
pub use input::{
    mk_input_event_post, take_events, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP,
    INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL,
};
pub use misc::{mk_debug, mk_idle_ms, mk_yield, slept_ms, take_debug, yields};
pub use reply::{mk_ipc_reply, take_reply};
pub use time::{mk_uptime_ms, Deadline};

// The real libc's register accessors and DMA barriers, mounted so the driver
// files under test reach a device through the code they ship with.
#[path = "../../libc/src/broker/dma_order.rs"]
mod dma_order;
#[path = "../../libc/src/broker/mmio_access.rs"]
mod mmio_access;
pub use dma_order::{dma_mb, dma_rmb, dma_wmb};
pub use mmio_access::{
    mmio_read16, mmio_read32, mmio_read64, mmio_read8, mmio_write16, mmio_write32, mmio_write64,
    mmio_write8,
};
