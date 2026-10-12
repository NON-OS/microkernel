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
pub mod dmar_scope;
mod ivrs;
pub mod ivrs_walk;
mod hmat_decode;
mod hmat_load;
pub mod hmat_entry;
pub mod hmat_query;
mod hpet;
mod ivhd_entry;
mod ivhd_kind;
pub mod ivhd_scope;
mod lpit_counter;
mod lpit_decode;
pub mod lpit_entry;
mod lpit_load;
mod mcfg;
mod numa_extra;
mod srat;
mod srat_decode;
pub mod slit_matrix;
pub mod srat_entry;
pub mod wdat_action;
pub mod wdat_decode;
pub mod wdat_entry;
mod wdat_load;
pub mod wdat_run;

pub use dmar::{foreign_segment_units, parse_dmar, remap_unit_bases, remap_unit_scopes};
pub use dmar_scope::{translated_by_some, UnitScope};
pub use hpet::parse_hpet;
pub use ivrs::{amd_iommu_bases, amd_iommu_spans, parse_ivrs};
pub use hmat_load::{hmat_attached_initiator, hmat_value, parse_hmat};
pub use lpit_counter::{lpit_residency, CounterError};
pub use lpit_load::{lpit_state, parse_lpit};
pub use mcfg::parse_mcfg;
pub use numa_extra::{numa_distance, numa_initiators, parse_slit};
pub use srat::parse_srat;
pub use wdat_load::{parse_wdat, wdat_action, wdat_table};
