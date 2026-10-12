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

//! An IORT node: its header (type, 16-bit length, revision, identifier,
//! ID mapping count and where the mappings start within the node) and
//! what its type carries.

use super::mapping::Mapping;
use alloc::vec::Vec;

pub const HEADER: usize = 16;

pub const ITS_GROUP: u8 = 0;
pub const NAMED_COMPONENT: u8 = 1;
pub const ROOT_COMPLEX: u8 = 2;
pub const SMMU_V3: u8 = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    ItsGroup {
        its_ids: Vec<u32>,
    },
    NamedComponent {
        flags: u32,
        cca: u32,
        address_limit: u8,
        name: Vec<u8>,
    },
    RootComplex {
        cca: u32,
        ats: u32,
        segment: u32,
        address_limit: Option<u8>,
    },
    Smmuv3 {
        base: u64,
        flags: u32,
        model: u32,
        /// Event, PRI, global error and sync GSIVs, 0 where not wired.
        gsivs: [u32; 4],
        proximity: Option<u32>,
        id_mapping_index: Option<u32>,
    },
    /// SMMUv1/v2, PMCG, RMR and later types: carried by number, unread.
    Other(u8),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Where the node starts in the IORT: what output references name.
    pub offset: u32,
    pub revision: u8,
    pub id: u32,
    pub kind: Kind,
    pub mappings: Vec<Mapping>,
}
