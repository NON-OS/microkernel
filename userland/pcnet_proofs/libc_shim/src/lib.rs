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

//! The things the included driver files take from `nonos_libc`, with the
//! broker calls recorded per thread. Each test runs on its own thread, so
//! nothing here needs a lock.

mod deadline;
mod debug;
mod dma;
mod entropy;
mod grants;

pub use deadline::Deadline;
pub use debug::{logged, mk_debug};
pub use dma::MK_DMA_MAP_DMA32;
pub use dma::{dma_flags_asked, mk_dma_map, script_dma, DmaMapOut, MK_DMA_MAP_COHERENT};
pub use entropy::{crypto_random, entropy};
pub use grants::{given_back, mk_device_release, mk_dma_unmap, mk_mmio_unmap};
