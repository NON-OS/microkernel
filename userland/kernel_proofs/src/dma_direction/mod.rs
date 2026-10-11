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

/*
 * Which way a DMA grant's data may move once an IOMMU confines the device:
 * the broker's direction rules are included by path, beside the flags and
 * the VT-d capability bits other modules already include.
 */

pub use crate::dma_flags as flags;
pub use crate::iommu_command::behaviour;
#[path = "../../../../src/hardware/broker/dma/direction.rs"]
pub mod direction;
mod tests;
mod zlr_tests;
