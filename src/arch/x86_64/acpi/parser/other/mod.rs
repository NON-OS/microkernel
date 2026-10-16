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

pub mod acpi_path;
mod dmar;
pub mod dmar_andd;
mod dmar_namespace_lookup;
pub mod dmar_namespace_scope;
mod dmar_namespace_store;
pub mod dmar_namespace_walk;
pub mod dmar_rmrr;
mod dmar_rmrr_store;
pub mod dmar_scope;
pub mod dmar_scope_walk;
mod dmar_source_store;
mod dmar_units;
mod hpet;
pub mod ivhd_alias;
mod ivhd_alias_store;
mod ivhd_entry;
mod ivhd_kind;
pub mod ivhd_scope;
pub mod ivhd_special;
mod ivrs;
pub mod ivrs_walk;
mod mcfg;
mod srat;
mod srat_memory;

pub use dmar::parse_dmar;
pub use dmar_namespace_lookup::{namespace_devices, namespace_dropped, namespace_requester};
pub use dmar_rmrr_store::{rmrr_dropped, rmrr_regions};
pub use dmar_scope::{translated_by_some, UnitScope};
pub use dmar_source_store::{hpet_source, ioapic_source, source_scopes_dropped};
pub use dmar_units::{foreign_segment_units, remap_unit_bases, remap_unit_scopes};
pub use hpet::parse_hpet;
pub use ivhd_alias_store::{amd_iommu_aliased, amd_iommu_special};
pub use ivrs::{amd_iommu_bases, amd_iommu_spans, parse_ivrs};
pub use mcfg::parse_mcfg;
pub use srat::parse_srat;
pub use srat_memory::{parse_memory_affinity, parse_x2apic_affinity};
