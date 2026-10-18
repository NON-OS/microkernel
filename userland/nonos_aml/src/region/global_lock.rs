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

//! The Global Lock's protocol on the FACS lock word (section 5.2.10.1): bit 0
//! is Pending, bit 1 is Owned. Each step returns the word to compare-and-swap
//! in, so the caller's atomic operation does the rest.

const PENDING: u32 = 1;
const OWNED: u32 = 2;

/// From the word as read: the word to swap in, and whether that takes the
/// lock. If firmware owns it, the swap marks it pending instead, and the
/// caller waits for firmware's release before trying again.
pub fn acquire_step(old: u32) -> (u32, bool) {
    if old & OWNED != 0 {
        (old | PENDING, false)
    } else {
        ((old & !PENDING) | OWNED, true)
    }
}

/// From the word as read: the word to swap in, and whether firmware was
/// waiting, in which case the caller sets GBL_RLS to tell it.
pub fn release_step(old: u32) -> (u32, bool) {
    (old & !(OWNED | PENDING), old & PENDING != 0)
}
