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

//! What a pseudo-terminal's slave side may read: the bytes the discipline
//! let through, and in canonical mode where each line ends, an end of file
//! (Ctrl-D on an empty line) being a line of no bytes. Pure.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

pub use crate::linux::console::queue_piece::Taken;

/// The most held unread, n_tty's own buffer size; a writer waits past it.
pub const MOST: usize = 4096;

#[derive(Default)]
pub struct Input {
    /// The line being edited, in canonical mode, not yet readable.
    pub line: Vec<u8>,
    pub ready: VecDeque<u8>,
    /// The length of each finished line in `ready`, oldest first.
    pub ends: VecDeque<usize>,
}

impl Input {
    /// True when the discipline takes no more until the slave reads.
    pub fn full(&self) -> bool {
        self.ready.len() + self.line.len() >= MOST
    }

    /// A finished line, readable whole: canonical mode's unit.
    pub fn finish(&mut self, extra: Option<u8>) {
        let mut line = core::mem::take(&mut self.line);
        line.extend(extra);
        self.ends.push_back(line.len());
        self.ready.extend(line);
    }

    /// One read of at most `most` bytes: in canonical mode never past the
    /// end of a line, and an end of file alone.
    pub fn take(&mut self, most: usize, canon: bool) -> Taken {
        if canon && self.ends.front() == Some(&0) {
            self.ends.pop_front();
            return Taken::Eof;
        }
        let line = self.ends.front().copied().filter(|_| canon).unwrap_or(self.ready.len());
        let n = line.min(most).min(self.ready.len());
        if n == 0 {
            return Taken::Empty;
        }
        let (out, mut gone): (Vec<u8>, usize) = (self.ready.drain(..n).collect(), n);
        while let Some(front) = self.ends.front_mut() {
            let k = gone.min(*front);
            (*front, gone) = (*front - k, gone - k);
            if *front != 0 || (k == 0 && canon) {
                break;
            }
            self.ends.pop_front();
        }
        Taken::Bytes(out)
    }
}
