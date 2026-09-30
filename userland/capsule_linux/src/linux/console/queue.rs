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

//! What the terminal sent and a guest has not read yet, in order: pieces
//! of bytes, and the ends of file typed between them. Pure, so the host
//! proofs hold the end-of-file marker and the reading to it.

use alloc::collections::VecDeque;

use super::queue_piece::{Piece, EOF, MOST};
use super::wipe::wipe;

pub struct Queue {
    pub(super) pieces: VecDeque<Piece>,
    pub(super) bytes: usize,
}

impl Queue {
    pub const fn new() -> Queue {
        Queue { pieces: VecDeque::new(), bytes: 0 }
    }

    /// True when no more is taken from the terminal until a guest reads.
    pub fn full(&self) -> bool {
        self.bytes >= MOST
    }

    /// Bytes waiting, as FIONREAD counts them.
    pub fn queued(&self) -> usize {
        self.bytes
    }

    /// True when a read would not wait: bytes, or an end of file.
    pub fn ready(&self) -> bool {
        !self.pieces.is_empty()
    }

    /// One message from the terminal: exactly [EOF] is an end of file, an
    /// empty one is nothing, anything else is bytes to read.
    pub fn push(&mut self, msg: &[u8]) {
        match msg {
            [] => {}
            [EOF] => self.pieces.push_back(Piece::Eof),
            _ => {
                self.bytes += msg.len();
                self.pieces.push_back(Piece::Bytes(msg.to_vec(), 0));
            }
        }
    }

    /// Everything unread dropped, and wiped, as TCSETSF asks.
    pub fn clear(&mut self) {
        while let Some(piece) = self.pieces.pop_front() {
            if let Piece::Bytes(mut bytes, _) = piece {
                wipe(&mut bytes);
            }
        }
        self.bytes = 0;
    }
}
