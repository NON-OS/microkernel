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

//! Joining ACL fragments into one L2CAP PDU (Vol 3, Part A, 7.2): a start
//! opens a PDU whose basic header states its length and continuations fill
//! it. `max` bounds what a link holds, since the peer picks the length.

use crate::error::WireError;
use crate::hci::Boundary;
use alloc::vec::Vec;

pub struct Reassembler {
    buf: Vec<u8>,
    open: bool,
    max: usize,
    dropped: u32,
}

impl Reassembler {
    pub fn new(max: usize) -> Self {
        Self { buf: Vec::new(), open: false, max, dropped: 0 }
    }
    /// Unfinished PDUs a later start replaced (a fragment lost below
    /// L2CAP); the new start is kept, so one loss costs one PDU.
    pub fn dropped(&self) -> u32 {
        self.dropped
    }

    /// The finished PDU, `None` while incomplete, or the refusal (then
    /// nothing is held).
    pub fn push(&mut self, b: Boundary, data: &[u8]) -> Result<Option<Vec<u8>>, WireError> {
        if b == Boundary::Continuing && !self.open {
            return Err(WireError::Sequence);
        }
        if b != Boundary::Continuing {
            if self.open {
                self.dropped = self.dropped.saturating_add(1);
            }
            self.buf.clear();
            self.open = true;
        }
        self.buf.extend_from_slice(data);
        let out = self.check();
        if !matches!(out, Ok(None)) {
            self.open = false;
            self.buf.clear();
        }
        out
    }

    fn check(&mut self) -> Result<Option<Vec<u8>>, WireError> {
        let Some(head) = self.buf.get(..2) else { return Ok(None) };
        let want = 4 + usize::from(u16::from_le_bytes([head[0], head[1]]));
        match (want > self.max, self.buf.len() > want) {
            (true, _) => return Err(WireError::TooLong),
            (_, true) => return Err(WireError::Length),
            _ => {}
        }
        Ok((self.buf.len() == want).then(|| core::mem::take(&mut self.buf)))
    }
}
