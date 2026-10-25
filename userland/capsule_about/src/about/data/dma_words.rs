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

//! What About's DMA row says. Device DMA is held only when the document says
//! a unit enforces, no mapping goes around it and no device is left reaching
//! all of memory (Doc::dma_held); every other document gets the owner's words,
//! exactly: with no IOMMU in service, under the no-IOMMU posture, or with a
//! unit in service and something beside it.

pub const CONFINED: &[u8] = b"device DMA held by the IOMMU";
pub const UNCONFINED: &[u8] = b"device DMA not confined by hardware";

pub const fn words(held: bool) -> &'static [u8] {
    if held {
        CONFINED
    } else {
        UNCONFINED
    }
}
