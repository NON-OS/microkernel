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

//! The things the included driver files take from `nonos_libc`, each
//! recorded per thread; every test runs on its own thread.

mod broker;
#[path = "../../../piix_ide_proofs/libc_shim/src/command_judge.rs"]
pub mod command_judge;
mod devices;
mod dma;
mod dma_at;
mod ipc;
mod pio;
mod script;

pub use broker::{
    mk_device_claim, mk_device_release, mk_pci_config_update, mk_pci_config_write, mk_pio_grant,
    mk_pio_release, PioGrantOut, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};
pub use devices::{
    mk_device_list, set_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI,
};
pub use dma::{
    mk_dma_map, mk_dma_unmap, script_dma, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32,
};
pub use dma_at::dma_at;
pub use ipc::{logged, mk_debug, mk_idle_ms, mk_ipc_reply, replies, Deadline};
pub use pio::{install_ports, mk_pio_read, mk_pio_write, Ports};
pub use script::{calls, script, Script};
