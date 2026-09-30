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

/*
 * A remapping unit's register accessors refuse every offset past the window.
 *
 * The kernel's accessors are included by path and run against a heap buffer
 * standing in for the mapped page. They checked offset + width against the
 * window, which wraps for an offset near usize::MAX where overflow checks are
 * off, so such an offset passed the check and was read. The check below fails
 * against that code.
 */

#[path = "../../../../src/arch/x86_64/iommu/unit/access.rs"]
pub mod access;
mod tests;
