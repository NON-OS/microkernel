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

//! A VMD BAR as (base, size), the shape the window choice takes.

use super::super::types::PciBar;

/// (0, 0) for an absent or I/O BAR.
pub(super) fn bar(bars: &[PciBar; 6], index: usize) -> (u64, u64) {
    match bars[index].address() {
        Some(base) if bars[index].is_memory() => (base.as_u64(), bars[index].size()),
        _ => (0, 0),
    }
}
