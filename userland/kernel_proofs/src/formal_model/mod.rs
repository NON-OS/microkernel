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


//! Each invariant `formal/` proves, checked on the Rust it abstracts. The
//! model's tables are read from `formal/Formal/Data`, the files
//! `formal/scripts/gen_tables.py` generates, and compared with the real
//! cap table on every registered syscall; the bit rules admission, the grant
//! handler, the map path and the surface registry run are included by path
//! and held to the predicate the matching Lean theorem states.

#[path = "../../../../src/security/capsule_manifest/verify/caps_bits.rs"]
mod caps_bits;
#[path = "../../../../src/security/image_ceiling/rule.rs"]
mod ceiling_rule;
#[path = "../../../../src/syscall/microkernel/capability/grant_rule.rs"]
pub(crate) mod grant_rule;
#[path = "../../../../src/process/address_space/protection.rs"]
mod protection;
#[path = "../../../../src/process/address_space/pte_flags.rs"]
mod pte_flags;
#[path = "../../../../src/kernel_core/surface_registry/share/rights_attach.rs"]
mod rights_attach;
#[path = "../../../../src/kernel_core/surface_registry/share/rights_rule.rs"]
mod rights_rule;
#[path = "../../../../src/kernel_core/surface_registry/share/rule.rs"]
mod share_rule;

mod lean_table;
mod words;

mod admission_tests;
mod allowlist_tests;
mod capability_tests;
mod gate_tests;
mod isolation_tests;
mod wx_tests;
