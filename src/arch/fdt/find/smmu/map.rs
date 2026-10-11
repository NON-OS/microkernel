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

/// Entries kept from one `iommu-map`. A host bridge splits its requester ids
/// between a few units at most; more than this is refused, not truncated.
pub const MAX_MAP_ENTRIES: usize = 8;

/// One `iommu-map` or `msi-map` entry: requester ids `[rid_base, rid_base +
/// length)` reach `phandle` as ids from `sid_base` on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MapEntry {
    pub rid_base: u32,
    pub phandle: u32,
    pub sid_base: u32,
    pub length: u32,
}

/// A host bridge's `iommu-map` or `msi-map`, with its mask applied to every
/// requester id before it is looked up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IommuMap {
    pub entries: [MapEntry; MAX_MAP_ENTRIES],
    pub count: usize,
    pub mask: u32,
}

impl IommuMap {
    /// The stream id `rid` carries at the unit `phandle`.
    pub fn stream_id(&self, rid: u16, phandle: u32) -> Option<u32> {
        self.output(rid, phandle)
    }

    /// The id `rid` is known by at `phandle`, a stream id at an SMMU or a
    /// device id at an ITS, or `None` when no entry sends it there. The first
    /// matching entry wins, as the `pci-iommu` and `pci-msi` bindings say.
    pub fn output(&self, rid: u16, phandle: u32) -> Option<u32> {
        let rid = u32::from(rid) & self.mask;
        let entry = self.entries[..self.count]
            .iter()
            .find(|e| rid >= e.rid_base && rid - e.rid_base < e.length)?;
        (entry.phandle == phandle).then(|| entry.sid_base + (rid - entry.rid_base))
    }
}
