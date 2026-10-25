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

//! The DMA sync windows, one backend per architecture: a fence on x86_64,
//! whose bus snoops the caches; cache maintenance by address on aarch64; a
//! full FENCE on riscv64. `memory::dma::coherency` is their only user.

#[cfg(target_arch = "aarch64")]
#[path = "aarch64/dma_sync/mod.rs"]
mod imp;
#[cfg(target_arch = "riscv64")]
#[path = "riscv64/dma_sync.rs"]
mod imp;
#[cfg(target_arch = "x86_64")]
#[path = "x86_64/dma_sync.rs"]
mod imp;

pub(crate) use imp::{sync_for_cpu, sync_for_device};
