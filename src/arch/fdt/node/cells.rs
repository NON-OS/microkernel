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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::limits::{DEFAULT_ADDRESS_CELLS, DEFAULT_SIZE_CELLS, MAX_CELLS};
use crate::arch::fdt::property::Property;

/// A bus's `#address-cells` and `#size-cells`: how its children's `reg` and
/// its own `ranges` are laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cells {
    pub address: u32,
    pub size: u32,
}

impl Cells {
    pub const DEFAULT: Self = Self { address: DEFAULT_ADDRESS_CELLS, size: DEFAULT_SIZE_CELLS };

    /// What an unreadable count is recorded as. Past `MAX_CELLS`, so every
    /// `reg` and `ranges` under it reads as nothing: a hostile count costs
    /// that subtree its addresses, not the walk.
    const UNREADABLE: u32 = MAX_CELLS + 1;

    /// Fold `prop` in if it is one of the two counts.
    pub fn take(&mut self, prop: &Property) {
        let value = |p: &Property| p.cell_count().unwrap_or(Self::UNREADABLE);
        if prop.name == b"#address-cells" {
            self.address = value(prop);
        } else if prop.name == b"#size-cells" {
            self.size = value(prop);
        }
    }

    /// Both counts small enough to read a value into a u64.
    pub fn fit_u64(&self) -> Result<(), FdtError> {
        if self.address == 0 || self.address > 2 || self.size > 2 {
            return Err(FdtError::BadCells);
        }
        Ok(())
    }
}
