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

//! Cutting the server's byte stream into frames. The length field is judged
//! against the stage's cap as soon as its four bytes are in, before any
//! byte of the body is kept, so the client never holds more than one
//! frame of the size its stage allows.

use alloc::vec::Vec;
use nsh_wire::{wire_len, Expect, WireError, LEN_BYTES};

#[derive(Default)]
pub(crate) struct Framer {
    len: [u8; LEN_BYTES],
    have: usize,
    need: usize,
    body: Vec<u8>,
}

/// A whole frame: its length field and the bytes after it.
pub(crate) type Frame = ([u8; LEN_BYTES], Vec<u8>);

impl Framer {
    /// Take bytes from the front of `input` up to the end of one frame.
    /// `None` means `input` ran out first; what was taken is kept.
    pub(crate) fn fill(
        &mut self,
        input: &mut &[u8],
        expect: Expect,
    ) -> Result<Option<Frame>, WireError> {
        while self.have < LEN_BYTES {
            let Some((b, rest)) = input.split_first() else { return Ok(None) };
            self.len[self.have] = *b;
            self.have += 1;
            *input = rest;
            if self.have == LEN_BYTES {
                self.need = wire_len(self.len, expect)?;
                self.body = Vec::with_capacity(self.need);
            }
        }
        let take = (self.need - self.body.len()).min(input.len());
        self.body.extend_from_slice(&input[..take]);
        *input = &input[take..];
        if self.body.len() < self.need {
            return Ok(None);
        }
        self.have = 0;
        Ok(Some((self.len, core::mem::take(&mut self.body))))
    }
}
