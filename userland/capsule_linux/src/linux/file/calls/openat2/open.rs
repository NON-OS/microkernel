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

/* openat2's entry. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::flags::O_CLOEXEC;
use super::check::check;

pub(super) const OPEN_HOW: usize = 24;

pub(super) const NO_XDEV: u64 = 0x01;

pub(super) const NO_MAGICLINKS: u64 = 0x02;

pub(super) const NO_SYMLINKS: u64 = 0x04;

pub(super) const BENEATH: u64 = 0x08;

pub(super) const IN_ROOT: u64 = 0x10;

pub(super) const CACHED: u64 = 0x20;

pub(super) const O_TMPFILE: u64 = 0o20200000;

/* Every open flag Linux knows (VALID_OPEN_FLAGS). */
pub(super) const VALID: u64 = 0o37777703;

pub fn openat2(guest: &mut Guest, dirfd: u64, path_ptr: u64, how: u64, size: u64) -> u64 {
    match check(guest, dirfd, path_ptr, how, size) {
        Ok((named, flags, mode)) => {
            let got = super::super::super::open::open_named(guest, named, flags, mode);
            super::super::super::open::mark(guest, got, flags & O_CLOEXEC != 0);
            got
        }
        Err(e) => errno::fail(e),
    }
}

/* Whether `..` in the name climbs above where it starts. */
pub(super) fn escapes(name: &[u8]) -> bool {
    let mut depth = 0i64;
    for part in name.split(|b| *b == b'/') {
        match part {
            b"" | b"." => {}
            b".." => depth -= 1,
            _ => depth += 1,
        }
        if depth < 0 {
            return true;
        }
    }
    false
}
