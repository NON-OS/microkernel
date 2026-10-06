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

//! Reading a batched answer from net.nym back into whole messages.
//!
//! The answer is a run of records, each a flag byte, a little endian u32
//! length and that many bytes. A message too long for one answer comes as
//! several records: the first says more follows, each later one says it
//! continues. Every exit message is one SOCKS response with its own header,
//! so a piece handed on as though it were whole is read as a response that
//! does not parse, and the stream loses those bytes for good.

extern crate alloc;

use alloc::vec::Vec;

/// Flag byte and length.
pub const RECORD_HEADER: usize = 5;

/// These bytes continue a message an earlier record began.
pub const FLAG_CONT: u8 = 1;

/// The message goes on in a later record.
pub const FLAG_MORE: u8 = 2;

/// The longest message this will put back together. An exit answers in
/// reads of a few kilobytes; a message claiming more is abandoned rather
/// than allowed to grow without bound.
pub const MESSAGE_MAX: usize = 512 * 1024;

/// What one answer amounted to.
#[derive(Default)]
pub struct Unpacked {
    /// Every message completed, in the order they completed.
    pub messages: Vec<Vec<u8>>,
    /// Pieces that could not be placed: a continuation with nothing begun,
    /// a message abandoned part way, or one past the bound.
    pub dropped: usize,
    /// The answer stopped making sense part way through, and what followed
    /// that point was not read.
    pub malformed: bool,
}

/// A message being put back together across answers.
pub struct Joiner {
    partial: Vec<u8>,
    open: bool,
}

impl Default for Joiner {
    fn default() -> Self {
        Self::new()
    }
}

impl Joiner {
    pub const fn new() -> Self {
        Self { partial: Vec::new(), open: false }
    }

    /// Forget a message begun under a session that is gone.
    pub fn reset(&mut self) {
        self.partial = Vec::new();
        self.open = false;
    }

    /// Read every record in `answer`, returning the messages they complete.
    pub fn feed(&mut self, answer: &[u8]) -> Unpacked {
        let mut out = Unpacked::default();
        let mut at = 0usize;
        while at < answer.len() {
            let Some(head) = answer.get(at..at.saturating_add(RECORD_HEADER)) else {
                out.malformed = true;
                break;
            };
            let flags = head[0];
            let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
            let start = at + RECORD_HEADER;
            let Some(end) = start.checked_add(len) else {
                out.malformed = true;
                break;
            };
            let Some(bytes) = answer.get(start..end) else {
                out.malformed = true;
                break;
            };
            at = end;
            if flags & !(FLAG_CONT | FLAG_MORE) != 0 {
                out.malformed = true;
                break;
            }
            self.place(flags & FLAG_CONT != 0, flags & FLAG_MORE != 0, bytes, &mut out);
        }
        if out.malformed && self.open {
            // What followed the bad record may have been the rest of the
            // message being built, so it can no longer be finished.
            self.reset();
            out.dropped += 1;
        }
        out
    }

    fn place(&mut self, cont: bool, more: bool, bytes: &[u8], out: &mut Unpacked) {
        if !cont {
            if self.open {
                // A new message while one was unfinished: the rest of the old
                // one was lost on the way, and it cannot be completed now.
                self.reset();
                out.dropped += 1;
            }
            if !more {
                out.messages.push(bytes.to_vec());
                return;
            }
            if bytes.len() > MESSAGE_MAX {
                out.dropped += 1;
                return;
            }
            self.partial = bytes.to_vec();
            self.open = true;
            return;
        }
        if !self.open {
            out.dropped += 1;
            return;
        }
        if self.partial.len().saturating_add(bytes.len()) > MESSAGE_MAX {
            self.reset();
            out.dropped += 1;
            return;
        }
        self.partial.extend_from_slice(bytes);
        if !more {
            out.messages.push(core::mem::take(&mut self.partial));
            self.open = false;
        }
    }
}
