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

/* struct open_how read and checked as Linux checks it. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::flags::O_CREAT;
use super::open::{
    BENEATH, CACHED, IN_ROOT, NO_MAGICLINKS, NO_SYMLINKS, NO_XDEV, OPEN_HOW, O_TMPFILE, VALID,
};

/*
 * The flags, mode and RESOLVE_ rules of the struct open_how at `at`,
 * checked as Linux checks them.
 */
pub(super) fn open_how(guest: &Guest, at: u64, size: u64) -> Result<(u64, u64, u64), i64> {
    if (size as usize) < OPEN_HOW || size > 4096 {
        return Err(errno::EINVAL);
    }
    let raw = guest.read(at, size as usize).ok_or(errno::EFAULT)?;
    /* A larger struct from a newer libc is fine while the new part is zero. */
    if raw[OPEN_HOW..].iter().any(|b| *b != 0) {
        return Err(7); /* E2BIG */
    }
    let word = |i: usize| u64::from_le_bytes(raw[i * 8..i * 8 + 8].try_into().unwrap_or([0; 8]));
    let (flags, mode, rules) = (word(0), word(1), word(2));
    if flags & !VALID != 0
        || rules & !(NO_XDEV | NO_MAGICLINKS | NO_SYMLINKS | BENEATH | IN_ROOT | CACHED) != 0
    {
        return Err(errno::EINVAL);
    }
    if mode != 0 && flags & (O_CREAT | O_TMPFILE) == 0 || mode & !0o7777 != 0 {
        return Err(errno::EINVAL);
    }
    if rules & IN_ROOT != 0 {
        return Err(errno::EINVAL);
    }
    if rules & CACHED != 0 {
        return Err(errno::EAGAIN);
    }
    Ok((flags, mode, rules))
}
