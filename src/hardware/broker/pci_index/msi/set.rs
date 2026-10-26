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

//! The MSI side table written, by the ACPI broker alone (a function added
//! after the scan, one departed): one entry per id at most, set or cleared whole.

use alloc::vec::Vec;

/// `id`'s entry replaced by `value`, or removed when there is none.
pub(in crate::hardware::broker) fn set<T>(table: &mut Vec<(u64, T)>, id: u64, value: Option<T>) {
    table.retain(|(d, _)| *d != id);
    if let Some(v) = value {
        table.push((id, v));
    }
}
