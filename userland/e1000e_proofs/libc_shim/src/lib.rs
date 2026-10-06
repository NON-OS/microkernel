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

//! The things the included driver files take from `nonos_libc`, each backed
//! by something a test can set or read: the host clock, a switchable entropy
//! source, a per-thread console, a scripted config space and a record of
//! every grant given back.

mod clock;
mod debug;
mod entropy;
mod pci;
mod release;

pub use clock::{mk_idle_ms, Deadline};
pub use debug::{clear_log, logged, mk_debug};
pub use entropy::{crypto_random, entropy, Entropy};
pub use pci::{config_reads, mk_pci_config_read, set_ring_status};
pub use release::{given_back, mk_device_release, mk_dma_unmap, mk_mmio_unmap};
