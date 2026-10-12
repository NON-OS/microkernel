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
pub mod bmp_check;
mod dmar;
pub mod dmar_scope;
mod hmat_decode;
pub mod hmat_entry;
mod hmat_load;
pub mod hmat_query;
mod hpet;
mod ivhd_entry;
mod ivhd_kind;
pub mod ivhd_scope;
mod ivrs;
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
mod serial_console_choice;
mod serial_console_load;
pub mod shared_tables;
pub mod slit_matrix;
mod srat;
mod srat_decode;
pub mod srat_entry;
pub mod wdat_action;
pub mod wdat_decode;
pub mod wdat_entry;
mod wdat_load;
pub mod wdat_run;

pub use bgrt_load::{boot_logo, boot_logo_file, parse_bgrt};
pub use dmar::{foreign_segment_units, parse_dmar, remap_unit_bases, remap_unit_scopes};
pub use dmar_scope::{translated_by_some, UnitScope};
pub use hmat_load::{hmat_attached_initiator, hmat_value, parse_hmat};
pub use hpet::parse_hpet;
pub use ivrs::{amd_iommu_bases, amd_iommu_spans, parse_ivrs};
pub use lpit_counter::{lpit_residency, CounterError};
pub use lpit_load::{lpit_state, parse_lpit};
pub use mcfg::parse_mcfg;
pub use numa_extra::{numa_distance, numa_initiators, parse_slit};
pub use parse_all::parse_other_tables;
pub use pcct_load::{parse_pcct, pcct_table};
pub use serial_console_choice::{spcr_console_port, CONSOLE_FROM_SPCR};
pub use serial_console_load::{debug_ports, parse_serial_console, serial_console};
pub use srat::parse_srat;
pub use wdat_load::{parse_wdat, wdat_action, wdat_table};
