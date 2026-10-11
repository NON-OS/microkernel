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

/// An SMMUv3 the device tree describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmmuInfo {
    /// Register window: page 0 at `base`, page 1 at `base + 64 KiB`.
    pub base: u64,
    pub size: u64,
    /// What `iommu-map` and `iommus` entries name this SMMU by; zero when no
    /// node can refer to it.
    pub phandle: u32,
    /// `dma-coherent`: its table and queue walks snoop the CPU caches, so
    /// the kernel need not clean them to memory before the SMMU reads.
    pub coherent: bool,
}
