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

#[inline]
pub(super) const fn align_up(value: u64, align: u64) -> u64 {
    if align == 0 || align & (align - 1) != 0 {
        return value;
    }
    /*
     * saturating_add pinned the sum at u64::MAX and the mask then cleared the
     * low bits, so align_up(u64::MAX, 4096) returned 0xFFFFFFFFFFFFF000: an
     * address strictly below the one it was asked to round up. A caller
     * computing an end address from that gets a region which appears to end
     * before it begins, which is the direction that turns a bounds check into a
     * pass. Returning the value unchanged keeps the one thing align_up promises,
     * that its result is never below its input.
     */
    match value.checked_add(align - 1) {
        Some(sum) => sum & !(align - 1),
        None => value,
    }
}

#[inline]
pub(super) const fn align_down(value: u64, align: u64) -> u64 {
    if align == 0 || align & (align - 1) != 0 {
        return value;
    }
    value & !(align - 1)
}
