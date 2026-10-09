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

//! Messages delivered by the mixnet and not yet collected by the reader.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::protocol::batch::{put_record, FLAG_CONT, FLAG_MORE, RECORD_HEADER};

/// Messages held for a reader that has not collected them yet.
///
/// A response arrives as many messages, and the reader only asks between its
/// own writes, so a whole page can queue up before anything is taken. When
/// this fills the oldest is dropped, and dropping the oldest of a byte stream
/// leaves a hole the far end will never resend: everything after it waits for
/// bytes that are gone. Sized so that filling it means the reader has stopped
/// reading, not that the answer was large.
pub const RX_DEPTH: usize = 256;

/// Bytes the queue may hold across its messages. The count alone bounds
/// nothing when one reassembled message can run to half a megabyte.
pub const RX_BYTES_MAX: usize = 2 * 1024 * 1024;

struct Queued {
    bytes: Vec<u8>,
    /// The rest of a message whose front was already handed over.
    cont: bool,
}

/// What one read takes off the front of the queue.
pub struct Part {
    pub bytes: Vec<u8>,
    /// These bytes continue a message an earlier read began.
    pub cont: bool,
    /// The message goes on past these bytes; the rest is still queued.
    pub more: bool,
}

pub struct RxQueue {
    items: VecDeque<Queued>,
    bytes: usize,
}

impl Default for RxQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl RxQueue {
    pub const fn new() -> Self {
        Self { items: VecDeque::new(), bytes: 0 }
    }

    /// Messages held.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether nothing is held.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Bytes held across them.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// Queue one delivered message, making room by dropping the oldest.
    ///
    /// A message larger than the whole queue may hold is refused rather than
    /// let it empty the queue for nothing. Returns whether it was kept.
    pub fn push(&mut self, body: Vec<u8>) -> bool {
        if body.len() > RX_BYTES_MAX {
            return false;
        }
        while self.items.len() >= RX_DEPTH || self.bytes.saturating_add(body.len()) > RX_BYTES_MAX {
            let Some(gone) = self.items.pop_front() else { break };
            self.bytes = self.bytes.saturating_sub(gone.bytes.len());
        }
        self.bytes = self.bytes.saturating_add(body.len());
        self.items.push_back(Queued { bytes: body, cont: false });
        true
    }

    /// Take at most `limit` bytes of the next message, leaving the rest where
    /// a later read will find it.
    ///
    /// A reply is whatever the far end had to say and can be larger than one
    /// reply carries. Taking it whole or not at all meant a message that did
    /// not fit was popped and thrown away, and it was the long ones that did
    /// not fit: the acknowledgements and the small answers arrived, the page
    /// bodies were destroyed one hop from the reader.
    pub fn take(&mut self, limit: usize) -> Option<Vec<u8>> {
        self.take_part(limit).map(|part| part.bytes)
    }

    /// As `take`, also saying whether the bytes begin, continue or end a
    /// message. A reader handed the front of a message has to know the rest
    /// is coming, or it reads the front as the whole and the rest as noise.
    pub fn take_part(&mut self, limit: usize) -> Option<Part> {
        if limit == 0 {
            return None;
        }
        let mut item = self.items.pop_front()?;
        self.bytes = self.bytes.saturating_sub(item.bytes.len());
        let more = item.bytes.len() > limit;
        if more {
            let rest = item.bytes.split_off(limit);
            self.bytes = self.bytes.saturating_add(rest.len());
            self.items.push_front(Queued { bytes: rest, cont: true });
        }
        Some(Part { bytes: item.bytes, cont: item.cont, more })
    }

    /// Move as many messages as fit into `out` as records, each a flag byte,
    /// a length and the bytes, and return how much of `out` was written.
    ///
    /// One read used to carry one message, so an answer that had arrived as
    /// sixty messages took sixty round trips through every capsule between
    /// the gateway and the reader. A message that does not fit in the room
    /// left goes as its front, flagged as continuing, unless something is
    /// already written: then it waits whole for the next read.
    pub fn fill_records(&mut self, out: &mut [u8]) -> usize {
        let mut at = 0usize;
        loop {
            let room = out.len().saturating_sub(at).saturating_sub(RECORD_HEADER);
            if room == 0 {
                return at;
            }
            if self.is_empty() {
                return at;
            }
            let Some(front) = self.items.front() else { return at };
            if at > 0 && front.bytes.len() > room {
                return at;
            }
            let Some(part) = self.take_part(room) else { return at };
            let mut flags = 0u8;
            if part.cont {
                flags |= FLAG_CONT;
            }
            if part.more {
                flags |= FLAG_MORE;
            }
            match put_record(out, at, flags, &part.bytes) {
                Some(next) => at = next,
                None => return at,
            }
        }
    }
}
