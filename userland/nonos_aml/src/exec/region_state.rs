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

//! What an OperationRegion and a buffer field resolve to, kept once made.

use crate::namespace::Id;
use crate::object::Buf;
use crate::region::Space;
use alloc::rc::Rc;
use alloc::vec::Vec;

/// Where a PCI_Config region's function is: the host bridge's segment and
/// bus, then the _ADR of each device from the bridge down to the region's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PciPath {
    pub segment: u16,
    pub bus: u8,
    pub adrs: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Region {
    pub space: Space,
    pub base: u64,
    pub len: u64,
    pub pci: Option<PciPath>,
    /// A DataTableRegion's table, read from the copy the capsule holds
    /// rather than from memory, and never written.
    pub table: Option<Rc<Vec<u8>>>,
}

#[derive(Clone, Debug)]
pub struct BufferField {
    pub buf: Buf,
    pub bit: u64,
    pub bits: u32,
}

/// One register access, as the host sees it.
#[derive(Clone, Copy, Debug)]
pub struct RegionAccess<'a> {
    pub space: Space,
    pub region: Id,
    /// The region's base plus the access's byte offset in it; for
    /// PCI_Config, the offset in the function's configuration space.
    pub address: u64,
    pub width: u32,
    pub pci: Option<&'a PciPath>,
    /// The whole region the access falls in, so the host can ask for all of
    /// it at once.
    pub base: u64,
    pub len: u64,
}
