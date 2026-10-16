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

use crate::arch::fdt::endian::be_cells;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::property::Property;

/// A `/chosen` address, one cell or two as its length says: `/chosen` has
/// no `#address-cells` of its own to say which.
pub(super) fn bound(prop: &Property) -> Result<u64, FdtError> {
    match prop.data.len() {
        4 => be_cells(prop.data, 0, 1),
        8 => be_cells(prop.data, 0, 2),
        _ => Err(FdtError::BadCells),
    }
}
