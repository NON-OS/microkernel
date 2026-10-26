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

pub use super::kind::Kind;
use super::mapping::Mapping;
use alloc::vec::Vec;

pub(crate) const HEADER: usize = 16;

pub(crate) const ITS_GROUP: u8 = 0;
pub(crate) const NAMED_COMPONENT: u8 = 1;
pub(crate) const ROOT_COMPLEX: u8 = 2;
pub(crate) const SMMU_V1_V2: u8 = 3;
pub(crate) const SMMU_V3: u8 = 4;
/// Reserved memory ranges (IORT revision E.c and later, recalled).
pub(crate) const RMR: u8 = 6;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    /// Where the node starts in the IORT: what output references name.
    pub offset: u32,
    pub revision: u8,
    pub id: u32,
    pub kind: Kind,
    pub mappings: Vec<Mapping>,
}
