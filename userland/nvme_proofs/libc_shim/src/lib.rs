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

//! The calls the included NVMe driver files take from `nonos_libc`, on the
//! host. Everything is per thread, so each test has its own clock, memory
//! and record.

mod calls;
mod clock;
mod debug;
mod device;
mod dma;
mod grants;
mod irq;
mod pci;

pub use calls::{calls, record, refuse, refuse_only};
pub use clock::{fail_clock, mk_uptime_ms, on_tick, set_clock};
pub use debug::{logged, mk_debug};
pub use device::{
    mk_device_claim, mk_irq_bind, mk_mmio_map, set_bar0, IrqBindOut, MmioMapOut, MK_IRQ_BIND_MSIX,
};
pub use dma::{mk_dma_map, mk_dma_unmap, DmaMapOut};
pub use grants::{mk_device_release, mk_irq_unbind, mk_mmio_unmap};
pub use irq::{mk_irq_wait, mk_yield, raise, MK_IRQ_WAIT_TIMED_OUT, REFUSED_GRANT};
pub use pci::{
    bus_master, command, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};
