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

//! Which AMD-Vi entries lead to another table, for the walk that frees a
//! dead domain's tables. Pure.

use super::pte::{address, is_present, next_level};

/// The table a directory at `level` names, or `None` for a leaf (whose next
/// level is 0, or 7 for a large page), an empty entry, or a directory naming
/// any level but the next one down, which is not followed on a guess.
pub const fn child_table(entry: u64, level: u8) -> Option<u64> {
    if level >= 2 && is_present(entry) && next_level(entry) == level - 1 {
        Some(address(entry))
    } else {
        None
    }
}
