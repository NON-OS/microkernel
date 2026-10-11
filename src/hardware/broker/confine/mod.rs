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

//! One IOMMU domain per driver capsule. A device a capsule claims leaves the
//! identity domain for the capsule's own, which maps nothing until `MkDmaMap`
//! grants a buffer, so the device reaches that capsule's grants and faults on
//! everything else. A device no unit confines is refused (decide.rs).

mod attach;
mod decide;
mod detach;
mod iova;
mod iova_space;
mod join;
mod map;
mod posture;
mod refuse;
mod table;
mod unmap;
mod virtio;
mod virtio_rule;

pub(super) use attach::attach;
pub(super) use detach::{detach, detach_all};
pub(super) use map::{map, NoAddress};
pub(super) use unmap::unmap;
