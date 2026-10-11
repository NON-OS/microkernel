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

//! Locating a PL031 real-time clock.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

const PL031: &[&[u8]] = &[b"arm,pl031"];

/// The MMIO base of the first enabled PL031 in the tree, or `None` for a
/// board with no real-time clock.
pub fn find(fdt: &Fdt) -> Result<Option<u64>, FdtError> {
    for found in fdt.devices(PL031) {
        let (node, _) = found?;
        if let Some((base, _)) = fdt.mmio(&node, 0)? {
            return Ok(Some(base));
        }
    }
    Ok(None)
}
