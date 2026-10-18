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

//! The device tree's reserved DMA ranges: `/reserved-memory` children whose
//! `iommu-addresses` name a device and the IOVA it must find the region at
//! (the reserved-memory binding, recalled, unverified).

mod backref;
mod cells;
mod entry;
mod owner;
mod read;
mod refusal;
mod streams;

pub use entry::{Owner, TreeRange, MAX_STREAMS, MAX_TREE_RANGES};
pub use read::ranges;
pub use refusal::Refusal;
