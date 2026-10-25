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

pub mod bgrt_copy;
pub mod bgrt_entry;
mod bgrt_load;
mod bgrt_mode;
pub mod bmp_check;
mod dmar;
mod dmar_ats;
mod dmar_cover;
mod dmar_entry;
mod dmar_flags;
pub mod dmar_keep;
pub mod dmar_ioapic;
pub mod dmar_ioapic_source;
mod dmar_ioapic_table;
mod dmar_parse;
mod dmar_reserved;
mod dmar_translated;
mod dmar_units;
pub mod dmar_rmrr;
pub mod dmar_scope;
mod hmat_decode;
pub mod hmat_entry;
mod hmat_load;
pub mod hmat_query;
mod hpet;
mod ivhd_entry;
mod ivhd_kind;
pub mod ivhd_special;
mod ivhd_special_table;
pub mod ivhd_scope;
pub mod ivmd;
mod ivmd_block;
mod ivmd_table;
mod ivrs;
mod ivrs_left;
pub mod ivrs_walk;
mod lpit_counter;
mod lpit_decode;
pub mod lpit_entry;
mod lpit_load;
mod mcfg;
mod numa_extra;
mod parse_all;
mod pcct_load;
mod raw_table;
mod sdev_load;
mod serial_console_choice;
mod serial_console_load;
pub mod shared_tables;
pub mod slit_matrix;
mod srat;
mod srat_decode;
pub mod srat_entry;
pub mod unit_base;
mod unit_base_env;
pub mod wdat_action;
pub mod wdat_decode;
pub mod wdat_entry;
mod wdat_load;
pub mod wdat_run;

pub use bgrt_load::{boot_logo, boot_logo_file, parse_bgrt};
pub use bgrt_mode::firmware_mode;
pub use dmar::parse_dmar;
pub use dmar_ats::ats_structures;
pub use dmar_flags::{dmar_address_width, dmar_intr_remap, dmar_platform_opt_in, dmar_x2apic_opt_out};
pub use dmar_ioapic_table::{ioapic_named, ioapic_source};
pub use dmar_reserved::reserved_regions;
pub use dmar_scope::{translated_by_some, UnitScope};
pub use dmar_units::{remap_unit_bases, units_left_off, with_remap_unit_scopes};
pub use hmat_load::{hmat_attached_initiator, hmat_value, parse_hmat};
pub use hpet::parse_hpet;
pub use ivhd_special_table::amd_ioapic_requester;
pub use ivmd_table::amd_reserved_for;
pub use ivrs::{amd_iommu_bases, amd_iommu_spans, parse_ivrs};
pub use ivrs_left::amd_units_left_off;
pub use lpit_counter::{lpit_residency, CounterError};
pub use lpit_load::{lpit_state, parse_lpit};
pub use mcfg::parse_mcfg;
pub use numa_extra::{numa_distance, numa_initiators, parse_slit};
pub use parse_all::parse_other_tables;
pub use pcct_load::{parse_pcct, pcct_table};
pub use sdev_load::{parse_sdev, sdev_entries};
pub use serial_console_choice::{spcr_console_port, CONSOLE_FROM_SPCR};
pub use serial_console_load::{debug_ports, parse_serial_console, serial_console};
pub use srat::parse_srat;
pub use wdat_load::{parse_wdat, wdat_action, wdat_table};
