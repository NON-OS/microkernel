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

//! The three repeat offsets (RFC 8878 3.1.2.5).

/// Offset_Value to an offset, updating the three repeat offsets. Values 1 to 3
/// name a repeat, shifted by one when the sequence has no literals.
pub fn resolve(rep: &mut [usize; 3], value: usize, llen: usize) -> Option<usize> {
    if value > 3 {
        *rep = [value - 3, rep[0], rep[1]];
        return Some(rep[0]);
    }
    let idx = value - 1 + usize::from(llen == 0);
    let offset = match idx {
        0 => return Some(rep[0]),
        1 => rep[1],
        2 => rep[2],
        _ => rep[0].checked_sub(1).filter(|&o| o > 0)?,
    };
    if idx != 1 {
        rep[2] = rep[1];
    }
    rep[1] = rep[0];
    rep[0] = offset;
    Some(offset)
}
