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

//! The DMAR table's parsers (VT-d 3.x, chapter 8), held together under one
//! module so the other tables' list stays short. Each file keeps its place
//! beside the rest, where the host proofs mount it by path.

use super::{unit_base, unit_base_env};

#[path = "acpi_path.rs"]
pub mod acpi_path;
#[path = "dmar.rs"]
pub(super) mod dmar;
#[path = "dmar_andd.rs"]
pub mod dmar_andd;
#[path = "dmar_ats.rs"]
pub(super) mod dmar_ats;
#[path = "dmar_cover.rs"]
mod dmar_cover;
#[path = "dmar_entry.rs"]
mod dmar_entry;
#[path = "dmar_flags.rs"]
pub(super) mod dmar_flags;
#[path = "dmar_ioapic.rs"]
pub mod dmar_ioapic;
#[path = "dmar_ioapic_source.rs"]
pub mod dmar_ioapic_source;
#[path = "dmar_ioapic_table.rs"]
pub(super) mod dmar_ioapic_table;
#[path = "dmar_keep.rs"]
pub mod dmar_keep;
#[path = "dmar_namespace_lookup.rs"]
pub(super) mod dmar_namespace_lookup;
#[path = "dmar_namespace_scope.rs"]
pub mod dmar_namespace_scope;
#[path = "dmar_namespace_store.rs"]
mod dmar_namespace_store;
#[path = "dmar_namespace_walk.rs"]
pub mod dmar_namespace_walk;
#[path = "dmar_parse.rs"]
mod dmar_parse;
#[path = "dmar_rmrr.rs"]
pub mod dmar_rmrr;
#[path = "dmar_rmrr_names.rs"]
pub mod dmar_rmrr_names;
#[path = "dmar_rmrr_store.rs"]
pub(super) mod dmar_rmrr_store;
#[path = "dmar_scope.rs"]
pub mod dmar_scope;
#[path = "dmar_scope_count.rs"]
mod dmar_scope_count;
#[path = "dmar_scope_walk.rs"]
pub mod dmar_scope_walk;
#[path = "dmar_source_store.rs"]
pub(super) mod dmar_source_store;
#[path = "dmar_translated.rs"]
mod dmar_translated;
#[path = "dmar_units.rs"]
pub(super) mod dmar_units;
#[path = "dmar_walk.rs"]
pub mod dmar_walk;
