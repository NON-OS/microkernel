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

use alloc::vec;
use alloc::vec::Vec;

use crate::image::types::DecodeError;

/* DEFLATE back-references reach at most 32 KiB behind the output. */
const WINDOW: usize = 32 * 1024;
const MASK: usize = WINDOW - 1;

/* Receives inflated bytes as they are produced. `done` tells the inflater
 * that nothing further is wanted, so it stops without reading the rest. */
pub trait Sink {
    fn put(&mut self, b: u8) -> Result<(), DecodeError>;
    fn done(&self) -> bool;
}

/* The sliding window: every byte passes through it on its way to the sink,
 * so matches copy from the last 32 KiB and the full output never exists. */
pub(super) struct Out<'s, K: Sink> {
    win: Vec<u8>,
    pos: usize,
    sink: &'s mut K,
}

impl<'s, K: Sink> Out<'s, K> {
    pub fn new(sink: &'s mut K) -> Self {
        Self { win: vec![0u8; WINDOW], pos: 0, sink }
    }

    pub fn lit(&mut self, b: u8) -> Result<(), DecodeError> {
        self.win[self.pos & MASK] = b;
        self.pos += 1;
        self.sink.put(b)
    }

    pub fn copy(&mut self, dist: usize, len: usize) -> Result<(), DecodeError> {
        if dist == 0 || dist > self.pos || dist > WINDOW {
            return Err(DecodeError::Truncated);
        }
        for _ in 0..len {
            let b = self.win[(self.pos - dist) & MASK];
            self.lit(b)?;
        }
        Ok(())
    }

    pub fn done(&self) -> bool {
        self.sink.done()
    }
}
