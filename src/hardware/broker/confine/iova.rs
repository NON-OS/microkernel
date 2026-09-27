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

use super::table::Capsule;

/// Device addresses start above the first megabyte, so a driver that hands a
/// device a zero or small address faults instead of hitting a grant.
pub(super) const IOVA_BASE: u64 = 0x10_0000;
/// Every grant sits below 4 GiB, so a 32-bit descriptor can name any of them.
const IOVA_LIMIT: u64 = 1 << 32;

/// Take `length` bytes of the capsule's device address space. A bump pointer:
/// grants are rings and staging buffers mapped once, and the top one is given
/// back on unmap, so churn only strands space below a live grant.
pub(super) fn take(c: &mut Capsule, length: u64) -> Option<u64> {
    let start = c.next_iova;
    let end = start.checked_add(length).filter(|&e| e <= IOVA_LIMIT)?;
    c.next_iova = end;
    Some(start)
}

pub(super) fn give_back(c: &mut Capsule, iova: u64, length: u64) {
    if iova.checked_add(length) == Some(c.next_iova) {
        c.next_iova = iova;
    }
}
