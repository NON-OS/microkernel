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

/*
 * What a descriptor's handle holds: the description's number and how
 * it was opened.
 */

use core::sync::atomic::{AtomicU32, Ordering};

use crate::linux::guest::{Fd, Kind};

const APPEND: u32 = 1 << 31;

const READS: u32 = 1 << 30;

const FLAGS: u32 = APPEND | READS;

static NEXT: AtomicU32 = AtomicU32::new(1);

/* A new description's handle. */
pub fn fresh(append: bool, reads: bool) -> u32 {
    let id = NEXT.fetch_add(1, Ordering::Relaxed) & !FLAGS;
    id | if append { APPEND } else { 0 } | if reads { READS } else { 0 }
}

/* The description's number, for a file or directory descriptor. */
pub fn of(fd: &Fd) -> Option<u32> {
    matches!(fd.kind, Kind::File | Kind::Dir).then_some(fd.handle & !FLAGS)
}

pub fn appends(fd: &Fd) -> bool {
    fd.kind == Kind::File && fd.handle & APPEND != 0
}

/* Opened O_RDONLY or O_RDWR: a read of a write-only descriptor is EBADF. */
pub fn reads(fd: &Fd) -> bool {
    fd.kind == Kind::File && fd.handle & READS != 0
}
