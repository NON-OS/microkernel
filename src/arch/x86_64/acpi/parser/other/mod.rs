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

pub mod dmar_tables;
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
mod ivhd_special_table;
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

pub use dmar_tables::dmar_rmrr;
pub use exports::*;
