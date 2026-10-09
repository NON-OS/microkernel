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

//! The things the included igc driver files take from `nonos_libc`, on the
//! host: a millisecond clock and sleep, the entropy source the station
//! address is drawn from, the console the "igc:" lines go to, and the broker
//! releases a failed attempt must make.

mod broker;
mod clock;
mod console;
mod entropy;

pub use broker::{given_back, mk_device_release, mk_dma_unmap, mk_mmio_unmap};
pub use clock::{mk_idle_ms, Deadline};
pub use console::{mk_debug, said};
pub use entropy::{crypto_random, entropy, Entropy};

/// The PCI Command bits, as nonos_libc broker/pci.rs defines them.
pub const MK_PCI_CMD_MEMORY_SPACE: u16 = 1 << 1;
pub const MK_PCI_CMD_BUS_MASTER: u16 = 1 << 2;
