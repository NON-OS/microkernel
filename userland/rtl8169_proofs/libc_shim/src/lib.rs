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

//! The things the included driver files take from `nonos_libc`.
//!
//! `crypto_random` is the one that matters. The station address is drawn from
//! it, and the driver's promise is that when there is no entropy there is no
//! address programmed at all, rather than the factory one left in place. The
//! test switches it off to hold the driver to that.

mod debug;
mod entropy;
mod pci;
mod release;
mod time;

pub use debug::{logged, mk_debug};
pub use entropy::{crypto_random, entropy, Entropy};
pub use pci::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};
pub use release::{given_back, mk_device_release, mk_dma_unmap, mk_irq_unbind, mk_mmio_unmap};
pub use time::{mk_uptime_ms, Deadline};
