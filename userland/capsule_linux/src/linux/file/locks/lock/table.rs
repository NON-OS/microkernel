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

/* The family's locks and what each covers. */

use alloc::vec::Vec;
use core::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /* flock, by open file description. */
    Flock(u32),
    /* F_SETLK, by the kernel pid of the process. */
    Posix(u32),
    /* F_OFD_SETLK, by open file description. */
    Ofd(u32),
}

#[derive(Clone)]
pub struct Lock {
    pub file: Vec<u8>,
    pub owner: Owner,
    pub write: bool,
    /* Bytes [start, end); end is u64::MAX for "to the end, however long". */
    pub start: u64,
    pub end: u64,
}

pub(super) struct Table(pub(super) RefCell<Vec<Lock>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Table {}

pub(super) static LOCKS: Table = Table(RefCell::new(Vec::new()));

pub(super) fn record(a: Owner) -> bool {
    !matches!(a, Owner::Flock(_))
}
