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

//! The sectors vfs and the volume code name, placed on the kept disk. vfs
//! numbers the store from LBA 256 up to the plan, as the fixed layout does;
//! a table that puts the store elsewhere moves the window, and one that
//! makes it smaller ends it sooner.

use super::fixed::FIXED;
use super::places::Places;

/// Store sectors `lba..lba + n`, as vfs numbers them, on the disk; `None`
/// outside the window or past the store partition.
pub(crate) fn store_at(p: &Places, lba: u64, n: u64) -> Option<u64> {
    let off = lba.checked_sub(FIXED.store.first)?;
    let end = off.checked_add(n)?;
    (end <= p.store.sectors.min(FIXED.store.sectors)).then_some(p.store.first + off)
}

/// Sector `index` of the plan partition: 0 the plan, 1 the key header.
pub(crate) fn plan_at(p: &Places, index: u64) -> Option<u64> {
    (index < p.plan.sectors).then_some(p.plan.first + index)
}
