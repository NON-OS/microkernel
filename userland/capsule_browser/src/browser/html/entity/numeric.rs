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

use super::c1::c1;

/// A numeric reference, decimal or hexadecimal, for `push_decoded`.
///
/// Anything outside Unicode, in a surrogate range or naming a control is
/// refused, so the caller writes the reference back out as it arrived.
pub fn numeric(name: &str) -> Option<char> {
    let digits = name.strip_prefix('#')?;
    let code = match digits.strip_prefix(['x', 'X']) {
        Some(hex) if !hex.is_empty() => u32::from_str_radix(hex, 16).ok()?,
        Some(_) => return None,
        None if !digits.is_empty() => digits.parse::<u32>().ok()?,
        None => return None,
    };
    match char::from_u32(c1(code)) {
        Some(c) if !c.is_control() || c == '\n' || c == '\t' => Some(c),
        _ => None,
    }
}
