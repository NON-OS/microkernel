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
//! memory, and a device record carries the three fields discovery matches
//! on. Nothing here answers as a device would. There is no MkDmaUnmap and no
//! MkExit: an included file that called either, and so freed the surface's
//! frames, would not build.

mod debug;
mod dma;
mod pci;
mod release;
mod reply;

pub use debug::{mk_debug, said};
pub use dma::{mk_dma_map, DmaMapOut};
pub use pci::{mk_pci_config_write, on_pci_write, pci_writes, MK_PCI_CFG_COMMAND};
pub use pci::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE};
pub use release::{mk_device_release, on_release, releases};
pub use reply::{mk_ipc_reply, replies};

/// The fields of the broker's device record that discovery reads.
pub struct DeviceRecord {
    pub vendor: u16,
    pub device: u16,
    pub bus_kind: u8,
}

pub const BUS_KIND_PCI: u8 = 1;
