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

/* Each description's offset, shared by the descriptors on it. */

use alloc::vec::Vec;
use core::cell::RefCell;

use crate::linux::guest::{Fd, Kind};

use super::handle::of;

pub(super) struct Offsets(pub(super) RefCell<Vec<(u32, u64)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Offsets {}

static OFFSETS: Offsets = Offsets(RefCell::new(Vec::new()));

/* Where the next read or write of a file goes: its description's offset. */
pub fn pos(fd: &Fd) -> u64 {
    let Some(d) = of(fd).filter(|_| fd.kind == Kind::File) else {
        return fd.offset;
    };
    OFFSETS.0.borrow().iter().find(|(x, _)| *x == d).map_or(fd.offset, |(_, at)| *at)
}

/* Move the description's offset, and the descriptor's copy of it. */
pub fn set_pos(fd: &mut Fd, at: u64) {
    fd.offset = at;
    let Some(d) = of(fd).filter(|_| fd.kind == Kind::File) else {
        return;
    };
    let mut all = OFFSETS.0.borrow_mut();
    match all.iter_mut().find(|(x, _)| *x == d) {
        Some(entry) => entry.1 = at,
        None => all.push((d, at)),
    }
}

/* The description is closed everywhere: its offset goes with it. */
pub fn gone(d: u32) {
    OFFSETS.0.borrow_mut().retain(|(x, _)| *x != d);
}
