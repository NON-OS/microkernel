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

//! DMAR device scopes (VT-d 3.x, 8.3.1): the types DRHD and RMRR share,
//! walked in dmar_scope_walk.rs, parsed in dmar_parse.rs and resolved in
//! dmar_cover.rs. Pure, so the host proofs include it unchanged.

pub use super::dmar_cover::scope_covers;
pub use super::dmar_parse::parse_drhd;
pub use super::dmar_translated::{translated_by_some, Cover};

/// DRHD flags bit 0: the unit covers every PCI device in its segment not named
/// by another unit's scope. The spec requires such a unit to be listed last.
pub const FLAG_INCLUDE_PCI_ALL: u8 = 1 << 0;
/// Device scope type 1: a PCI endpoint.
pub const SCOPE_ENDPOINT: u8 = 1;
/// Device scope type 2: a PCI bridge and every device below it.
pub const SCOPE_BRIDGE: u8 = 2;
/// Scopes kept per DRHD: all a u8 count holds (one root bus has 256 functions).
/// A unit naming more is marked truncated.
pub const MAX_SCOPES: usize = 255;
/// Path hops kept per scope; a longer path is marked unresolvable.
pub const MAX_PATH: usize = 4;

pub(super) const DRHD_HEADER_LEN: usize = 16;

/// One PCI device scope: its type, the bus its path starts on, and the
/// (device, function) hops of the path.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Scope {
    pub kind: u8,
    pub start_bus: u8,
    pub path: [(u8, u8); MAX_PATH],
    pub path_len: u8,
}

impl Scope {
    pub const EMPTY: Scope = Scope { kind: 0, start_bus: 0, path: [(0, 0); MAX_PATH], path_len: 0 };
}

/// What a DRHD (or, with N its scope count, an RMRR) says it covers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitScope<const N: usize = MAX_SCOPES> {
    pub include_all: bool,
    pub segment: u16,
    pub scopes: [Scope; N],
    pub count: u8,
    /// A PCI scope was dropped (too many, or a path too long to keep), so a
    /// device not matched may still be this unit's.
    pub truncated: bool,
}

impl<const N: usize> UnitScope<N> {
    pub const EMPTY: Self = UnitScope {
        include_all: false,
        segment: 0,
        scopes: [Scope::EMPTY; N],
        count: 0,
        truncated: false,
    };
}
