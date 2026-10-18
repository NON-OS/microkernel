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

//! The things the included driver and serve files take from `nonos_libc`,
//! each recorded per thread; every test runs on its own thread.

mod devices;
mod ipc;

pub use devices::{mk_device_list, set_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
pub use ipc::{
    logged, mk_cap_check, mk_debug, mk_getpid, mk_idle_ms, mk_ipc_recv_from, mk_ipc_send, mk_yield, recv_ready,
    sent, store_write, Deadline,
};
