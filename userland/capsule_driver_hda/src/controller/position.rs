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
//! Where the stream engine is in the ring.
//!
//! LPIB is the engine's link position; the DMA position buffer is the same
//! count written to memory by the controller. Linux reads Intel playback from
//! the buffer (`POS_FIX_SKL`, and `POS_FIX_AUTO` tries it first), falling
//! back to LPIB if the buffer never moves (`azx_position_ok`), and reads AMD
//! and graphics controllers by LPIB. The refill only needs to know which
//! period the engine is in, and either source tells it; the choice decides
//! which one is trusted.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    Buffer,
    Lpib,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub source: Source,
    /// The buffer was seen to move, so it is trusted from here on.
    proven: bool,
}

impl Position {
    pub const fn new(buffer: bool) -> Self {
        Position { source: if buffer { Source::Buffer } else { Source::Lpib }, proven: false }
    }

    /// The position to refill by, given both readings. A buffer that still
    /// reads zero while LPIB has moved is not being written, and is dropped
    /// for LPIB for the rest of the run.
    pub fn pick(&mut self, buffer: u32, lpib: u32, ring: u32) -> u32 {
        if self.source == Source::Buffer && !self.proven {
            if buffer != 0 {
                self.proven = true;
            } else if lpib != 0 {
                self.source = Source::Lpib;
            }
        }
        let pos = match self.source {
            Source::Buffer => buffer,
            Source::Lpib => lpib,
        };
        if ring == 0 || pos < ring {
            pos
        } else {
            pos % ring
        }
    }
}
