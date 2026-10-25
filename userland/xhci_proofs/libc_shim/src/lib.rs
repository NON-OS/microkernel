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

//! What the included driver files take from `nonos_libc`.
//!
//! Standing in, not stubbing out: `Deadline` keeps real time so a wait that
//! the specification says must give up really gives up, and DMA grants are
//! host memory a test can reach as the controller would (`grants`). The
//! broker serves one device per test thread: a register window the test
//! offers, the interrupt kinds it says the device has, and an ordered log
//! of everything given back (`log`). The server's replies are kept for the
//! test (`ipc`). Registers are reached through the real libc's ordered
//! accessors (`ordered`), as on the target.

mod broker;
mod clock;
mod device_dma;
mod dma;
mod dma_refuse;
mod grants;
mod ipc;
mod irq;
mod irq_wait;
mod log;
mod mmio;
mod ordered;
mod pci_command;
mod process;
mod unmap;
/// libc's own park policy, run against the stand-ins here.
#[path = "../../../libc/src/wait/mod.rs"]
pub mod wait;

pub use broker::{
    mk_debug, mk_device_claim, mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND,
    MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};
pub use clock::{mk_idle_ms, mk_yield, slept_ms, yields, Deadline};
pub use device_dma::{device_read64, DmaFault};
pub use dma::{mk_dma_map, DmaMapOut};
pub use dma_refuse::refuse_dma_after;
pub use grants::dma_host;
pub use ipc::{mk_ipc_recv_from, mk_ipc_reply, mk_ipc_send, recv_ready, take_replies};
pub use irq::{irq_tried, mk_irq_bind, offer_irq, IrqBindOut, MK_IRQ_BIND_MSI, MK_IRQ_BIND_MSIX};
pub use irq_wait::{irq_waited_on, mk_irq_ack, mk_irq_unbind, mk_irq_wait};
pub use log::{start_log, take_log, Call, Logged};
pub use mmio::{mk_mmio_map, mk_mmio_unmap, offer_mmio, MmioMapOut, MMIO_GRANT};
pub use ordered::*;
pub use pci_command::{pci_command_reg, PciCommandReg};
pub use process::{mk_pid_alive, pid_exits};
pub use unmap::mk_dma_unmap;
