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

//! The MSI side table read, free of kernel state for the host proofs.

/// The entry for `id`, if any.
pub(in crate::hardware::broker) fn of<T: Copy>(table: &[(u64, T)], id: u64) -> Option<T> {
    table.iter().find(|(d, _)| *d == id).map(|(_, v)| *v)
}
