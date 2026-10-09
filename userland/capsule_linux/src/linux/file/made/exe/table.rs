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

/* The family's record of what each process runs. */

use alloc::vec::Vec;
use core::cell::RefCell;

use super::shape::Exe;

const COMM: usize = 15;

pub(super) struct Table(pub(super) RefCell<Vec<(u32, Exe)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Table {}

static TABLE: Table = Table(RefCell::new(Vec::new()));

/* The image `pid` now runs, replacing what it ran before. */
pub fn record(pid: u32, exe: Exe) {
    let mut all = TABLE.0.borrow_mut();
    all.retain(|(p, _)| *p != pid);
    all.push((pid, exe));
}

pub fn of(pid: u32) -> Option<Exe> {
    TABLE.0.borrow().iter().find(|(p, _)| *p == pid).map(|(_, e)| e.clone())
}

/*
 * `pid` has left the family: its record goes with it. /proc answers only
 * for the processes the family holds, so nothing reads the record again;
 * kept, every exec a family ever made stayed here for its whole life, and
 * a shell running commands in a loop grew the table, and every lookup in
 * it, without end.
 */
pub fn forget(pid: u32) {
    TABLE.0.borrow_mut().retain(|(p, _)| *p != pid);
}

pub fn comm_of(name: &[u8]) -> Vec<u8> {
    let last = name.rsplit(|b| *b == b'/').next().unwrap_or(name);
    last[..last.len().min(COMM)].to_vec()
}
