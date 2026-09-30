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

//! Page-by-page byte transfer between kernel buffers and the physical frames
//! behind a user virtual address.

use super::error::UsercopyError;
use super::walk::{translate_read, translate_write, UserLeaf};
use crate::memory::layout::DIRECTMAP_BASE;
use crate::smp::SERVE_UNIT;

pub(super) fn copy_from_user_directmap(user_ptr: u64, dst: &mut [u8]) -> Result<(), UsercopyError> {
    transfer(user_ptr, dst.len(), translate_read, |leaf, off, n| {
        let src = (DIRECTMAP_BASE + leaf.phys_base + leaf.offset) as *const u8;
        /*
         * SAFETY: ek@nonos.systems - `leaf` came from `translate_read`
         * so the page is mapped, USER, and reachable through the
         * directmap, and `n` does not exceed the bytes left in it.
         */
        unsafe { core::ptr::copy_nonoverlapping(src, dst[off..].as_mut_ptr(), n) };
    })
}

pub(super) fn copy_to_user_directmap(user_ptr: u64, src: &[u8]) -> Result<(), UsercopyError> {
    transfer(user_ptr, src.len(), translate_write, |leaf, off, n| {
        let dst = (DIRECTMAP_BASE + leaf.phys_base + leaf.offset) as *mut u8;
        // SAFETY: ek@nonos.systems — `leaf` came from `translate_write`
        // so the page is mapped, USER, WRITABLE.
        unsafe { core::ptr::copy_nonoverlapping(src[off..].as_ptr(), dst, n) };
    })
}

fn transfer<T, S>(user_ptr: u64, len: usize, translate: T, mut step: S) -> Result<(), UsercopyError>
where
    T: Fn(u64) -> Result<UserLeaf, UsercopyError>,
    S: FnMut(&UserLeaf, usize, usize),
{
    let mut cursor = 0usize;
    let mut since_serve = 0usize;
    while cursor < len {
        /*
         * A copy runs with interrupts masked and can be megabytes, so it
         * answers TLB shootdowns once per `SERVE_UNIT`, and each piece is
         * capped at one unit so a 2 MiB or 1 GiB leaf is still translated
         * afresh per unit. An ack lets the other cpu free the frames it
         * unmapped, so no leaf is held here: the last was used up by `step`,
         * and the next is walked afresh from the page tables after it.
         */
        if since_serve >= SERVE_UNIT {
            crate::smp::serve_shootdowns();
            since_serve = 0;
        }
        let va = user_ptr.checked_add(cursor as u64).ok_or(UsercopyError::AddressOverflow)?;
        let leaf = translate(va)?;
        let remaining = leaf.bytes_remaining_in_page() as usize;
        let n = remaining.min(len - cursor).min(SERVE_UNIT);
        step(&leaf, cursor, n);
        cursor += n;
        since_serve += n;
    }
    Ok(())
}
