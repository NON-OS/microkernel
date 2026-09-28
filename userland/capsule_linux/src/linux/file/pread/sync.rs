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

/* preadv and pwritev, and a write put in the store as RWF_DSYNC asks. */

use crate::linux::call;
use crate::linux::guest::Guest;

use super::vector::vectored;

pub fn preadv(guest: &mut Guest, fd: u64, iov: u64, count: u64, at: u64, flags: u64) -> u64 {
    vectored(guest, fd, at, flags, |g| call::readv(g, fd, iov, count))
}

pub fn pwritev(guest: &mut Guest, fd: u64, iov: u64, count: u64, at: u64, flags: u64) -> u64 {
    vectored(guest, fd, at, flags, |g| call::writev(g, fd, iov, count))
}
