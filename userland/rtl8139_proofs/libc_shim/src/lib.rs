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

//! What the included driver files take from `nonos_libc`: the device list,
//! the claim and port grant, the port calls a modelled part answers, the
//! entropy the station address is drawn from, the clock, the console, the
//! PCI Command register, the releases, each logged, and the IPC calls the
//! server loop makes.

mod claim;
mod clock;
mod debug;
mod devices;
mod dma;
mod entropy;
mod grants;
mod ipc;
mod pci;
mod port;
mod steps;

pub use claim::{mk_device_claim, mk_pio_grant};
pub use clock::{uptime_ms, Deadline};
pub use debug::{logged, mk_debug};
pub use devices::{mk_device_list, present, Bar, DeviceRecord};
pub use devices::{BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
pub use dma::{dma_flags, line_up, mk_dma_map, DmaMapOut, MK_DMA_MAP_DMA32};
pub use entropy::{crypto_random, entropy, Entropy};
pub use grants::{given_back, mk_device_release, mk_dma_unmap, mk_pio_release, PioGrantOut};
pub use ipc::{line_up_requests, mk_ipc_recv_from, mk_ipc_reply, recv_ready, replies, SERVED};
pub use pci::{command, mk_pci_config_write, refuse_bus_master, set_command};
pub use pci::{MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};
pub use port::{attach, mk_pio_read, mk_pio_write, Attached, Port};
pub use steps::{seen, watch, Part, Seen, Step};
