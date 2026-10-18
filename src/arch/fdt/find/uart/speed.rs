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
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// The speed `stdout-path` asks for (its options start with the baud, as
/// in `115200n8`), else the node's `current-speed`.
pub(super) fn baud(fdt: &Fdt, node: &NodeRef, options: &[u8]) -> Result<Option<u32>, FdtError> {
    if let Some(baud) = leading_number(options) {
        return Ok(Some(baud));
    }
    Ok(match fdt.property(node, b"current-speed")? {
        Some(p) if p.data.len() == 4 => Some(p.u32()?).filter(|&b| b != 0),
        _ => None,
    })
}

fn leading_number(text: &[u8]) -> Option<u32> {
    let mut digits = text.iter().take_while(|b| b.is_ascii_digit());
    let value =
        digits.try_fold(0u32, |acc, &d| acc.checked_mul(10)?.checked_add((d - b'0') as u32));
    value.filter(|&v| v != 0)
}
