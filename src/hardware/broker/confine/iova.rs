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

use super::iova_space::{after_give_back, place};
use super::table::Capsule;

pub(super) use super::iova_space::IOVA_BASE;

/// Take `length` bytes of the capsule's device address space. A bump pointer:
/// grants are rings and staging buffers mapped once, and the top one is given
/// back on unmap, so churn only strands space below a live grant. Placement,
/// including the step over the interrupt window, is `iova_space::place`.
pub(super) fn take(c: &mut Capsule, length: u64) -> Option<u64> {
    let (start, end) = place(c.next_iova, length)?;
    c.next_iova = end;
    Some(start)
}

pub(super) fn give_back(c: &mut Capsule, iova: u64, length: u64) {
    c.next_iova = after_give_back(c.next_iova, iova, length);
}
