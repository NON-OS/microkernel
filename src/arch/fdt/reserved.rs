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

//! `/reserved-memory`: firmware-owned ranges described as nodes rather than
//! in the reservation block.

use super::error::FdtError;
use super::parser::Fdt;

impl<'a> Fdt<'a> {
    /// Fill `out` with every fixed region under `/reserved-memory`, each
    /// child's every `reg` entry, and return how many. A child with only a
    /// `size` asks the OS to pick a place and reserves nothing yet. Running
    /// out of room is an error: a reservation dropped is memory handed out
    /// that firmware still uses.
    pub fn reserved_memory(&self, out: &mut [(u64, u64)]) -> Result<usize, FdtError> {
        let Some(parent) = self.find_path(b"/reserved-memory")? else {
            return Ok(0);
        };
        let mut filled = 0usize;
        for child in self.children(&parent)? {
            let child = child?;
            let mut index = 0usize;
            while let Some(region) = self.mmio(&child, index)? {
                // The Pi leaves its bootloader nodes at `reg = <0 0 0>` until
                // the firmware fills them in; an empty range holds nothing.
                if region.1 != 0 {
                    let slot = out.get_mut(filled).ok_or(FdtError::TooLarge)?;
                    *slot = region;
                    filled += 1;
                }
                index += 1;
            }
        }
        Ok(filled)
    }
}
