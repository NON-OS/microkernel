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

//! The byte scan behind `read_user_string`.

use super::error::UsercopyError;
use super::walk::translate_read;
use crate::memory::layout::DIRECTMAP_BASE;

/// Bytes copied into `buf` before the terminator, or an error.
pub(super) fn scan_until_nul(
    user_ptr: u64,
    safe_len: usize,
    buf: &mut [u8],
) -> Result<usize, UsercopyError> {
    let mut cursor = 0usize;
    while cursor < safe_len {
        let va = user_ptr.checked_add(cursor as u64).ok_or(UsercopyError::AddressOverflow)?;
        let leaf = translate_read(va)?;
        let bytes_in_page = leaf.bytes_remaining_in_page() as usize;
        let take = bytes_in_page.min(safe_len - cursor);
        let src = (DIRECTMAP_BASE + leaf.phys_base + leaf.offset) as *const u8;
        for i in 0..take {
            /*
             * SAFETY: ek@nonos.systems - `leaf` came from
             * `translate_read`, so the read targets directmap memory
             * covered by the caller's page tables. Volatile, so the
             * compiler does not collapse the scan.
             */
            let byte = unsafe { core::ptr::read_volatile(src.add(i)) };
            if byte == 0 {
                return Ok(cursor + i);
            }
            buf[cursor + i] = byte;
        }
        cursor += take;
    }
    Err(UsercopyError::NotTerminated)
}
