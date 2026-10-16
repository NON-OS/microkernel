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

//! What a node declares, kept per depth while the walk is inside it.

/// Cells a node's children use when it does not say (the specification's
/// defaults).
const ADDRESS_CELLS: u32 = 2;
const SIZE_CELLS: u32 = 1;

#[derive(Clone, Copy)]
pub(super) struct Level {
    pub cells: (u32, u32),
    /// The node's own `reg` is a CPU address: every ancestor below the
    /// root maps its children one to one (an empty `ranges`).
    pub reaches: bool,
    pub ranges_empty: bool,
    pub inherited_irq_parent: Option<u32>,
    pub irq_parent: Option<u32>,
    pub dw: bool,
    pub okay: bool,
    pub reg: Option<(u64, u64)>,
    pub irq: ([u32; 3], u8),
    pub clock: Option<u32>,
    pub bus_hz: Option<u32>,
}

impl Level {
    pub fn root() -> Self {
        Self::child_of(None, true)
    }

    pub fn child_of(irq_parent: Option<u32>, reaches: bool) -> Self {
        Self {
            cells: (ADDRESS_CELLS, SIZE_CELLS),
            reaches,
            ranges_empty: false,
            inherited_irq_parent: irq_parent,
            irq_parent: None,
            dw: false,
            okay: true,
            reg: None,
            irq: ([0; 3], 0),
            clock: None,
            bus_hz: None,
        }
    }

    /// Whether this node's children's `reg` is a CPU address.
    pub fn passes_down(&self, is_root: bool) -> bool {
        is_root || (self.reaches && self.ranges_empty)
    }

    pub fn irq_parent_in_force(&self) -> Option<u32> {
        self.irq_parent.or(self.inherited_irq_parent)
    }
}
