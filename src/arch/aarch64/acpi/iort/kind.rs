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

//! What each IORT node type carries, as read.

use alloc::vec::Vec;

/// An interrupt as an SMMUv1/v2 node lists it: its GSIV, and whether it
/// is edge-triggered (flags bit 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Irq {
    pub gsiv: u32,
    pub edge: bool,
}

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
    Smmuv2 {
        base: u64,
        span: u64,
        /// 0 v1, 1 v2, then particular parts (recalled).
        model: u32,
        flags: u32,
        /// The two global interrupts, then one per context bank, then PMU.
        global: Vec<Irq>,
        context: Vec<Irq>,
        pmu: Vec<Irq>,
    },
    /// Memory an SMMU must keep mapped one to one for the stream IDs this
    /// node's mappings name: (base, length), each non-empty and unwrapped.
    Rmr {
        flags: u32,
        ranges: Vec<(u64, u64)>,
    },
    /// PMCG and later types: carried by number, unread.
    Other(u8),
}
