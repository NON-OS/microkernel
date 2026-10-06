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

//! Which of a process's standard streams reach a terminal, and the size of
//! that terminal in cells. Set by the parent that drains the process's
//! output, read by the process, forgotten when it exits.

use alloc::collections::BTreeMap;
use spin::Mutex;

/// Bit 0 stdin, bit 1 stdout, bit 2 stderr.
pub const STREAMS_ALL: u8 = 0b111;

#[derive(Clone, Copy)]
pub struct Tty {
    pub streams: u8,
    pub cols: u16,
    pub rows: u16,
}

static TABLE: Mutex<BTreeMap<u32, Tty>> = Mutex::new(BTreeMap::new());

/// Record what `pid`'s streams are on. No streams at all clears the entry,
/// which is what a stage whose output is redirected to a file gets.
pub fn set(pid: u32, tty: Tty) {
    let mut table = TABLE.lock();
    if tty.streams == 0 {
        table.remove(&pid);
    } else {
        table.insert(pid, tty);
    }
}

pub fn get(pid: u32) -> Option<Tty> {
    TABLE.lock().get(&pid).copied()
}

/// Called as a process is finalized, so a pid reused later starts with no
/// terminal it never had.
pub fn forget(pid: u32) {
    TABLE.lock().remove(&pid);
}
