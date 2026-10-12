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

pub mod attributes;
mod boot_devices;
mod boot_map;
mod control;
mod direct_map;
mod geometry;
pub mod granule;
mod image_bounds;
mod image_map;
mod l2_pool;
mod l3_tables;
mod map;
mod ram_map;
mod secondary;
mod state;
pub mod table;
mod tables_build;
mod tlb;
pub mod translation;
mod ttbr;
mod unmap;

pub use attributes::{MemoryType, PageAttributes};
pub use boot_map::init_mmu;
pub use direct_map::direct_map_end;
pub use geometry::KERNEL_SPACE_START;
pub use granule::{Granule, GRANULE_16K, GRANULE_4K, GRANULE_64K};
pub use map::map_page;
pub use secondary::publish_for_secondaries;
pub use table::PageTable;
pub use tlb::{flush_tlb_all, flush_tlb_asid, flush_tlb_page};
pub use translation::{phys_to_virt, virt_to_phys};
pub use ttbr::{read_ttbr0, read_ttbr1, set_ttbr0, set_ttbr1};
pub use unmap::unmap_page;
