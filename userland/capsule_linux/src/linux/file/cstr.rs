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

//! A NUL-terminated string out of a guest, a page at a time. Written over
//! the guest's memory, so the host proofs hold what it answers.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::memory::Memory;
use crate::linux::guest::{page_down, PAGE};

/// Bytes up to the terminator, or nothing when the string is not
/// terminated inside `max` or reaches memory the guest does not hold.
pub fn read_cstr(mem: &impl Memory, addr: u64, max: usize) -> Option<Vec<u8>> {
    cstr(mem, addr, max).ok()
}

/// Bytes up to the terminator, or why not, as Linux's strncpy_from_user
/// and getname answer: EFAULT for a null pointer or a string that reaches
/// memory the guest does not hold first, ENAMETOOLONG for one with no
/// terminator inside `max` bytes. Nothing past `max` is read.
pub fn cstr(mem: &impl Memory, addr: u64, max: usize) -> Result<Vec<u8>, i64> {
    if addr == 0 {
        return Err(errno::EFAULT);
    }
    let mut out: Vec<u8> = Vec::new();
    let mut at = addr;
    while out.len() <= max {
        let page_end = page_down(at).checked_add(PAGE).ok_or(errno::EFAULT)?;
        let room = (max + 1 - out.len()) as u64;
        let take = (page_end.saturating_sub(at)).min(room);
        let chunk = mem.read_at(at, take as usize).ok_or(errno::EFAULT)?;
        if let Some(i) = chunk.iter().position(|b| *b == 0) {
            out.extend_from_slice(&chunk[..i]);
            return Ok(out);
        }
        out.extend_from_slice(&chunk);
        at = page_end;
    }
    Err(errno::ENAMETOOLONG)
}
