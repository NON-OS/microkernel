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

//! What the parsers of the other ACPI tables give the rest of the kernel.

pub use super::dmar_tables::dmar::parse_dmar;
pub use super::dmar_tables::dmar_ats::ats_structures;
pub use super::dmar_tables::dmar_flags::{
    dmar_address_width, dmar_intr_remap, dmar_platform_opt_in, dmar_x2apic_opt_out,
};
pub use super::dmar_tables::dmar_ioapic_table::{ioapic_named, ioapic_source};
pub use super::dmar_tables::dmar_namespace_lookup::{
    namespace_devices, namespace_dropped, namespace_requester,
};
pub use super::dmar_tables::dmar_rmrr_names::{rmrr_barred_by, rmrr_names, rmrr_own};
pub use super::dmar_tables::dmar_rmrr_store::{rmrr_barred, rmrr_dropped, rmrr_regions};
pub use super::dmar_tables::dmar_scope::{translated_by_some, Cover, UnitScope};
pub use super::dmar_tables::dmar_source_store::{
    hpet_source, ioapic_unit_source, source_scopes_dropped,
};
pub use super::dmar_tables::dmar_units::{
    remap_unit_bases, units_left_off, with_remap_unit_scopes,
};
pub use super::hmat_load::{hmat_value, parse_hmat};
pub use super::hpet::parse_hpet;
pub use super::ivhd_alias_store::{amd_iommu_aliased, amd_iommu_special};
pub use super::ivhd_special_table::amd_ioapic_requester;
pub use super::ivmd_table::amd_reserved_for;
pub use super::ivrs::{amd_iommu_bases, amd_iommu_spans, parse_ivrs};
pub use super::ivrs_left::amd_units_left_off;
pub use super::lpit_counter::{lpit_residency, CounterError};
pub use super::lpit_load::{lpit_state, parse_lpit};
pub use super::mcfg::parse_mcfg;
pub use super::numa_extra::{numa_distance, numa_initiators, parse_slit};
pub use super::parse_all::parse_other_tables;
pub use super::pcct_load::parse_pcct;
pub use super::sdev_load::{parse_sdev, sdev_entries};
pub use super::serial_console_load::parse_serial_console;
pub use super::srat::parse_srat;
pub use super::wdat_load::{parse_wdat, wdat_action, wdat_table};
