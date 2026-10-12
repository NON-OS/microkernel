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

use crate::arch::fdt::node::DmaWindow;

use super::kind::PlatformKind;

/// Register windows kept per device; the Pi 4's have one or two.
pub const MAX_WINDOWS: usize = 4;
/// Platform devices kept from one tree.
pub const MAX_PLATFORM: usize = 8;

/// A GIC SPI and its trigger, as the tree states them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlatformIrq {
    pub intid: u32,
    pub edge: bool,
}

/// One allowlisted node: where its registers are, the interrupt it raises
/// and where its DMA lands, all in CPU terms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlatformDevice {
    pub kind: PlatformKind,
    /// `(base, size)` CPU physical, in `reg` order.
    pub windows: [(u64, u64); MAX_WINDOWS],
    pub window_count: usize,
    /// `None` when its first interrupt is not a GIC SPI.
    pub irq: Option<PlatformIrq>,
    /// `None` when no bus between it and the root maps its DMA anywhere.
    pub dma: Option<DmaWindow>,
}

impl PlatformDevice {
    pub const EMPTY: Self = Self {
        kind: PlatformKind::Sdhci,
        windows: [(0, 0); MAX_WINDOWS],
        window_count: 0,
        irq: None,
        dma: None,
    };
}
