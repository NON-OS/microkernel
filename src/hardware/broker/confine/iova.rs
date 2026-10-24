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

use super::avoid::place_avoiding;
use super::iova_holes::take_hole;
use super::table::Capsule;

pub(super) use super::iova_space::IOVA_BASE;

/// Take `length` bytes of the capsule's device address space: the first hole
/// a freed grant left that fits, else fresh space above the bump pointer.
/// Placement steps over the interrupt window (`iova_space::place`) and every
/// RMRR region and reserved range the capsule holds (`avoid`); holes touch
/// none of them, since no grant does.
pub(super) fn take(c: &mut Capsule, length: u64) -> Option<u64> {
    if let Some(start) = take_hole(&mut c.holes, length) {
        return Some(start);
    }
    let spans = c.reserved_spans();
    let (start, end) = place_avoiding(c.next_iova, length, &spans)?;
    c.next_iova = end;
    Some(start)
}

pub(super) fn give_back(c: &mut Capsule, iova: u64, length: u64) {
    super::iova_holes::give_back(&mut c.next_iova, &mut c.holes, iova, length);
}
