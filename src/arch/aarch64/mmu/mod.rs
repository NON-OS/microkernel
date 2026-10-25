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

pub mod asid;
pub mod attributes;
mod boot_devices;
mod boot_map;
mod control;
mod direct_map;
mod geometry;
pub mod granule;
mod image_bounds;
mod image_map;
mod kernel_va;
mod l2_pool;
mod l3_tables;
mod map;
pub mod one_to_one;
pub mod pointer;
mod ram_map;
mod secondary;
mod state;
pub mod table;
mod tables_build;
mod text_guard;
mod tlb;
pub mod tlbi_operand;
pub(in crate::arch::aarch64) mod trampoline;
pub mod translation;
mod ttbr;
mod uncached_block;
mod unmap;

pub use attributes::{MemoryType, PageAttributes};
pub use boot_map::init_mmu;
pub use direct_map::direct_map_end;
pub use geometry::KERNEL_SPACE_START;
pub use granule::{Granule, GRANULE_16K, GRANULE_4K, GRANULE_64K};
pub(in crate::arch::aarch64) use kernel_va::{devices_high, high_mapped};
pub use kernel_va::{image_pa, io, runs_high};
pub use map::map_page;
pub use secondary::{publish_for_secondaries, published_root};
pub use table::PageTable;
pub use tlb::{flush_tlb_all, flush_tlb_asid, flush_tlb_page};
pub use translation::virt_to_phys;
pub use ttbr::{read_ttbr0, read_ttbr1, set_ttbr0, set_ttbr1};
pub use uncached_block::make_block_uncached;
pub use unmap::unmap_page;
