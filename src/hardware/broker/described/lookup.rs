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

use super::iommus::DescribedIommus;
use super::state::DESCRIBED;
use super::window::BusWindow;

/// `None` for a device this module did not publish. `Some(None)` for one
/// no bus maps DMA for: it is given no grant.
pub(in crate::hardware::broker) fn dma_window(device_id: u64) -> Option<Option<BusWindow>> {
    DESCRIBED.read().iter().find(|e| e.device_id == device_id).map(|e| e.dma)
}

/// `None` for a device this module did not publish; else its `iommus`.
pub(in crate::hardware::broker) fn iommus(device_id: u64) -> Option<DescribedIommus> {
    DESCRIBED.read().iter().find(|e| e.device_id == device_id).map(|e| e.iommus)
}
