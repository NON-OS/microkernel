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
pub mod bgrt_copy;
pub mod bgrt_entry;
mod bgrt_load;
mod bgrt_mode;
pub mod bmp_check;
mod dmar;
pub mod dmar_andd;
mod dmar_namespace_lookup;
pub mod dmar_namespace_scope;
mod dmar_namespace_store;
pub mod dmar_namespace_walk;
pub mod dmar_rmrr;
pub mod dmar_rmrr_names;
mod dmar_rmrr_store;
pub mod dmar_scope;
pub mod dmar_scope_walk;
mod dmar_source_store;
mod dmar_units;
pub mod dmar_walk;
mod exports;
mod hmat_decode;
pub mod hmat_entry;
mod hmat_load;
pub mod hmat_query;
mod hpet;
pub mod ivhd_alias;
mod ivhd_alias_store;
mod ivhd_entry;
mod ivhd_kind;
pub mod ivhd_scope;
pub mod ivhd_special;
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
mod sdev_load;
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

pub use exports::*;
