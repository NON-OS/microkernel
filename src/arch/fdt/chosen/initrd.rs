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
use crate::arch::fdt::parser::Fdt;

use super::address::bound;

impl<'a> Fdt<'a> {
    /// The initrd the loader placed, as `(start, end)` with `end` exclusive.
    /// A loader may write a bound as one cell or two; its length says which.
    pub fn initrd(&self) -> Result<Option<(u64, u64)>, FdtError> {
        let Some(chosen) = self.find_path(b"/chosen")? else {
            return Ok(None);
        };
        let start = self.property(&chosen, b"linux,initrd-start")?;
        let end = self.property(&chosen, b"linux,initrd-end")?;
        match (start, end) {
            (None, None) => Ok(None),
            (Some(start), Some(end)) => {
                let (start, end) = (bound(&start)?, bound(&end)?);
                if end < start {
                    return Err(FdtError::OutOfBounds);
                }
                Ok(Some((start, end)))
            }
            // Half a range is not a range; guessing the other half could
            // reserve the wrong memory or none.
            _ => Err(FdtError::OutOfBounds),
        }
    }

    /// A raw `/chosen` property such as `rng-seed`, `kaslr-seed` or
    /// `bootargs`.
    pub fn chosen_bytes(&self, name: &[u8]) -> Result<Option<&'a [u8]>, FdtError> {
        let Some(chosen) = self.find_path(b"/chosen")? else {
            return Ok(None);
        };
        Ok(self.property(&chosen, name)?.map(|p| p.data))
    }
}
