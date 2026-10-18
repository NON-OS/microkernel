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

//! One reserved DMA range (the ruling).

/// `len` bytes the device reaches at `iova`, translated to `phys`. Identity
/// when the two are equal, which is what an RMRR always is; the tree's
/// `iommu-addresses` may give either.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReservedRange {
    pub iova: u64,
    pub phys: u64,
    pub len: u64,
}

impl ReservedRange {
    pub const fn is_identity(&self) -> bool {
        self.iova == self.phys
    }

    /// The IOVA span, `None` when it wraps.
    pub const fn iova_end(&self) -> Option<u64> {
        self.iova.checked_add(self.len)
    }

    /// The physical span, `None` when it wraps.
    pub const fn phys_end(&self) -> Option<u64> {
        self.phys.checked_add(self.len)
    }
}
