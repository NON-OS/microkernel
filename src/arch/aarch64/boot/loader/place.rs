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

//! Where a handoff may lie: wholly inside memory the device tree names, and
//! aligned for its fields. Pure, so the proofs run it over every edge.

/// Every field of the handoff is at most eight bytes wide.
pub(crate) const HANDOFF_ALIGN: u64 = 8;

/// True when `[at, at + len)` is non-empty, aligned and inside one of
/// `regions` (base, size), each of any type: the loader puts the handoff in
/// memory it allocated, which the tree may mark reserved.
pub(crate) fn placed(regions: impl Iterator<Item = (u64, u64)>, at: u64, len: u64) -> bool {
    let Some(end) = at.checked_add(len) else {
        return false;
    };
    if at == 0 || len == 0 || !at.is_multiple_of(HANDOFF_ALIGN) {
        return false;
    }
    regions
        .into_iter()
        .any(|(base, size)| base.checked_add(size).is_some_and(|top| base <= at && end <= top))
}
