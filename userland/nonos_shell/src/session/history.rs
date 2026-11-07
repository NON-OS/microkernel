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
//! The lines a session has run, newest last, and `history`, which numbers
//! them from 0 as the terminal does. A session keeps its last `MAX_HISTORY`
//! lines and the first `MAX_HISTORY_LINE` bytes of each, so no peer can
//! grow it without bound.

use super::Session;
use crate::out::ShellOut;
use crate::util::format_u64;

/// The most lines a session keeps.
pub const MAX_HISTORY: usize = 64;
/// The most bytes of one line a session keeps.
pub const MAX_HISTORY_LINE: usize = 4096;

impl Session {
    /// Keep `line` as the newest, dropping the oldest when full. A line of
    /// spaces alone is not kept.
    pub(super) fn remember(&mut self, line: &[u8]) {
        if line.iter().all(|c| *c == b' ' || *c == b'\t') {
            return;
        }
        if self.history.len() >= MAX_HISTORY {
            self.history.remove(0);
        }
        let keep = line.len().min(MAX_HISTORY_LINE);
        self.history.push(line[..keep].to_vec());
    }
}

/// `history`: each kept line after its number.
pub(super) fn history(s: &Session, out: &mut dyn ShellOut, w: &[&[u8]]) -> bool {
    if w.len() > 1 {
        out.writeln(b"usage: history");
        return false;
    }
    for (i, line) in s.history.iter().enumerate() {
        let mut num = [0u8; 20];
        let n = format_u64(i as u64, &mut num);
        out.writeln(&[&num[..n], b": ", &line[..]].concat());
    }
    true
}
