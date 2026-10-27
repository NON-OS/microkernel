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

//! The paths a request names, read from its payload before any handler
//! runs, so the guard (guard.rs) is asked in one place for every op that
//! takes a path. The layouts are the handlers' own: past the caller pid, a
//! u8 length and the path, and for a rename or a copy a second such pair
//! right after the first.

use crate::protocol::{
    OP_CHMOD, OP_COPY, OP_DIRSTAT, OP_JOURNAL_TOUCH, OP_LIST, OP_MKDIR, OP_OPEN, OP_RENAME,
    OP_RMDIR, OP_STAT, OP_STORE_INSTALL, OP_STORE_PERSIST, OP_STORE_REMOVE, OP_STORE_UNINSTALL,
    OP_TRUNCATE, OP_UNLINK,
};

/// The raw paths `op` names in `rest`, the payload past the caller pid. A
/// payload too short to hold one names none here, and its handler refuses
/// it for the same shortness.
pub fn named(op: u16, rest: &[u8]) -> [Option<&[u8]>; 2] {
    match op {
        OP_OPEN | OP_STAT | OP_LIST | OP_MKDIR | OP_UNLINK | OP_RMDIR | OP_TRUNCATE | OP_CHMOD
        | OP_DIRSTAT | OP_STORE_PERSIST | OP_STORE_REMOVE | OP_STORE_INSTALL
        | OP_STORE_UNINSTALL | OP_JOURNAL_TOUCH => [one(rest).map(|(path, _)| path), None],
        OP_RENAME | OP_COPY => match one(rest) {
            Some((first, after)) => [Some(first), one(after).map(|(path, _)| path)],
            None => [None, None],
        },
        _ => [None, None],
    }
}

/// A `<u8 len><path>` pair at the start of `bytes`, and what follows it.
fn one(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    let (&len, tail) = bytes.split_first()?;
    let path = tail.get(..len as usize)?;
    Some((path, &tail[len as usize..]))
}
