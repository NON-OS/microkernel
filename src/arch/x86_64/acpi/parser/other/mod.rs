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

mod dmar;
mod dmar_cover;
mod dmar_flags;
pub mod dmar_ioapic;
pub mod dmar_ioapic_source;
mod dmar_ioapic_table;
mod dmar_parse;
mod dmar_reserved;
mod dmar_translated;
mod dmar_units;
pub mod dmar_rmrr;
pub mod dmar_scope;
pub mod ivmd;
mod ivmd_block;
mod ivmd_table;
mod ivrs;
pub mod ivrs_walk;
mod hpet;
mod ivhd_entry;
mod ivhd_kind;
pub mod ivhd_scope;
mod mcfg;
mod srat;
mod srat_memory;

pub use dmar::parse_dmar;
pub use dmar_units::{foreign_segment_units, remap_unit_bases, remap_unit_scopes};
pub use dmar_flags::dmar_platform_opt_in;
pub use dmar_ioapic_table::{ioapic_named, ioapic_source};
pub use dmar_reserved::reserved_regions;
pub use dmar_scope::{translated_by_some, UnitScope};
pub use hpet::parse_hpet;
pub use ivmd_table::amd_reserved_for;
pub use ivrs::{amd_iommu_bases, amd_iommu_spans, parse_ivrs};
pub use mcfg::parse_mcfg;
pub use srat::parse_srat;
pub use srat_memory::{parse_memory_affinity, parse_x2apic_affinity};
