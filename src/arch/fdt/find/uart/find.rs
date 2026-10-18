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

use super::compatible::NEEDLES;
use super::describe::describe;
use super::info::UartInfo;

/// The console: the UART `/chosen` names when it is one we drive, otherwise
/// the first enabled one in the tree. A `stdout-path` naming a UART we do not
/// drive, or naming nothing, is passed over rather than trusted with the
/// wrong register layout; `Fdt::stdout` is where the reason can be read.
pub fn find(fdt: &Fdt) -> Result<Option<UartInfo>, FdtError> {
    if let Ok(Some((node, options))) = fdt.stdout() {
        if let (Some(needle), true) = (fdt.compatible_with(&node, NEEDLES)?, fdt.is_enabled(&node)?)
        {
            if let Some(info) = describe(fdt, &node, needle, options)? {
                return Ok(Some(info));
            }
        }
    }
    for found in fdt.devices(NEEDLES) {
        let (node, needle) = found?;
        if let Some(info) = describe(fdt, &node, needle, &[])? {
            return Ok(Some(info));
        }
    }
    Ok(None)
}
