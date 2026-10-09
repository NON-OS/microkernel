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

//! One read of the terminal's input, as a terminal in canonical mode
//! answers it: the bytes up to and including the next newline, never past
//! an end of file, and that end of file alone when it comes first.

use alloc::vec::Vec;

use super::queue::Queue;
use super::queue_piece::{Piece, Taken};
use super::wipe::wipe;

impl Queue {
    /// At most `most` bytes, an end of file, or Empty when nothing waits.
    pub fn take(&mut self, most: usize) -> Taken {
        match self.pieces.front() {
            None => return Taken::Empty,
            Some(Piece::Eof) => {
                self.pieces.pop_front();
                return Taken::Eof;
            }
            Some(Piece::Bytes(..)) => {}
        }
        let mut out = Vec::new();
        while out.len() < most {
            let Some(Piece::Bytes(bytes, at)) = self.pieces.front_mut() else {
                break;
            };
            let rest = bytes.get(*at..).unwrap_or(&[]);
            let line = rest.iter().position(|&b| b == b'\n').map_or(rest.len(), |i| i + 1);
            let n = line.min(most - out.len());
            out.extend_from_slice(&rest[..n]);
            *at += n;
            let ended = out.last() == Some(&b'\n');
            if *at >= bytes.len() {
                wipe(bytes);
                self.pieces.pop_front();
            }
            if ended {
                break;
            }
        }
        self.bytes = self.bytes.saturating_sub(out.len());
        Taken::Bytes(out)
    }
}
