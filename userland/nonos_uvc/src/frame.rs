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
//! Frames put back together from payloads (header.rs reads each one's
//! header). A frame is handed up whole and only once; one with an error,
//! one cut short or one past its bound is dropped.

use alloc::vec::Vec;

pub enum Payload {
    /// Nothing complete yet.
    More,
    /// A frame finished: its bytes are `Assembler::frame`.
    Frame,
}

pub struct Assembler {
    pub(crate) max: usize,
    pub(crate) buf: Vec<u8>,
    pub(crate) fid: Option<bool>,
    pub(crate) bad: bool,
    pub(crate) done: Vec<u8>,
}

impl Assembler {
    /// Frames of at most `max` bytes (the commit's dwMaxVideoFrameSize).
    pub fn new(max: usize) -> Self {
        Self { max, buf: Vec::new(), fid: None, bad: false, done: Vec::new() }
    }

    /// The last whole frame.
    pub fn frame(&self) -> &[u8] {
        &self.done
    }

    pub(crate) fn finish(&mut self) -> Payload {
        let whole = !self.bad && !self.buf.is_empty();
        if whole {
            self.done = core::mem::take(&mut self.buf);
        }
        self.buf.clear();
        self.bad = false;
        if whole {
            Payload::Frame
        } else {
            Payload::More
        }
    }
}
