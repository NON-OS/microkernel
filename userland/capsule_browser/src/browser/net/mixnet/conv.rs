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

//! One conversation with a proxy, as state, with no call in it.
//!
//! A send used to wait on the proxy's answer, and while net.socks5 opened a
//! cold mixnet session (up to four net.nym calls of 15 s each) it waited for
//! up to 75 s on the thread that draws the window: the browser looked hung.
//! Now nothing waits longer than the poll wait. Bytes handed to `take` go out
//! in numbered frames; a frame that has not been answered stays `asked`,
//! number and bytes alike, and the next exchange asks it again unchanged.
//! The proxy keeps its answer to a number and gives it back for the same
//! bytes, so they reach the exit once however many times they are asked.
//!
//! While a frame is asked, nothing else is: not new bytes, which would go
//! under a number the proxy has not finished with, and not an empty poll,
//! which under that number would be read as the next exchange if the asked
//! frame never reached the proxy (a call the kernel turned away as busy), and
//! the bytes in it would be skipped. A read therefore asks the asked frame.
//!
//! Pure, so the proofs drive it against a proxy that answers late or never.

use alloc::vec::Vec;

use super::frames::{answer, lost, next_seq, numbered, reset, CARRY_MAX, FIRST_SEQ};

/// Bytes a conversation holds that have not gone in a frame yet. A page's
/// request is a few KiB and a TLS flight less; past this the caller is
/// writing without reading, and refusing is better than holding more.
pub const UNSENT_MAX: usize = 256 * 1024;

/// The frame asked and not yet answered.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Asked {
    /// The reset every conversation begins with.
    Reset,
    /// Stream bytes, under their number.
    Bytes { seq: u32, bytes: Vec<u8> },
}

/// Why bytes were not taken.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// The far end finished, or the proxy can no longer be asked.
    Finished,
    /// More is waiting to go than a conversation holds.
    Full,
}

/// Why the proxy can be asked nothing more, when that is not the far end
/// finishing: each is said to the reader for what it is, where all of them
/// used to read as the exit closing the connection.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Broke {
    /// A call was refused outright: no proxy at the port any more.
    Gone,
    /// It answered with bytes that are not an answer, or too many of them.
    Garbled,
    /// It said it holds no such conversation: it was restarted under it.
    Lost,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Conv {
    /// The stream every frame of this conversation names; 0 names none.
    stream: u32,
    /// The number of the next exchange. It moves on only when one is
    /// answered.
    seq: u32,
    /// Bytes the proxy has answered that the reader has not taken yet.
    pending: Vec<u8>,
    /// The proxy said the far end finished: nothing more will come.
    closed: bool,
    /// A call to the proxy was refused, or it answered with something that
    /// is not an answer: nothing more will come either.
    broken: bool,
    /// Why, once `broken`.
    why: Option<Broke>,
    /// Bytes taken and not yet put in a frame.
    unsent: Vec<u8>,
    asked: Option<Asked>,
}

impl Conv {
    /// A conversation about to begin. Its first frame is the reset, and
    /// nothing goes before the proxy has answered it: a greeting that
    /// overtook it would be carried into whatever the proxy still held.
    pub fn opening(stream: u32) -> Conv {
        Conv {
            stream,
            seq: FIRST_SEQ,
            pending: Vec::new(),
            closed: false,
            broken: false,
            why: None,
            unsent: Vec::new(),
            asked: Some(Asked::Reset),
        }
    }

    /// The stream this conversation names.
    pub fn stream(&self) -> u32 {
        self.stream
    }

    /// Bytes are waiting to go, or have gone and are not answered yet.
    pub fn sending(&self) -> bool {
        !self.over() && (self.asked.is_some() || !self.unsent.is_empty())
    }

    /// Nothing more will be sent or received, apart from what is held.
    pub fn over(&self) -> bool {
        self.closed || self.broken
    }

    /// Everything the far end sent has been read and nothing more will come.
    pub fn ended(&self) -> bool {
        self.over() && self.pending.is_empty()
    }

    /// Bytes have arrived that the reader has not taken.
    pub fn holding(&self) -> bool {
        !self.pending.is_empty()
    }

    /// Take `bytes` to send. They go in the frames that follow.
    pub fn take(&mut self, bytes: &[u8]) -> Result<(), Refusal> {
        if self.over() {
            return Err(Refusal::Finished);
        }
        if self.unsent.len().saturating_add(bytes.len()) > UNSENT_MAX {
            return Err(Refusal::Full);
        }
        self.unsent.extend_from_slice(bytes);
        Ok(())
    }

    /// The frame the next exchange carries: the asked frame again, else the
    /// next bytes waiting (which become the asked frame), else an empty
    /// poll asking whether anything has arrived.
    pub fn frame(&mut self) -> Vec<u8> {
        if self.asked.is_none() && !self.unsent.is_empty() {
            let n = self.unsent.len().min(CARRY_MAX);
            let bytes: Vec<u8> = self.unsent.drain(..n).collect();
            self.asked = Some(Asked::Bytes { seq: self.seq, bytes });
        }
        match &self.asked {
            Some(Asked::Reset) => reset(self.stream),
            Some(Asked::Bytes { seq, bytes }) => numbered(self.stream, *seq, bytes),
            None => numbered(self.stream, self.seq, &[]),
        }
    }

    /// The proxy answered the last frame with `raw`. False when that is not
    /// an answer at all, which ends the conversation.
    pub fn answered(&mut self, raw: &[u8]) -> bool {
        if self.asked == Some(Asked::Reset) {
            /* Whatever it says, the reset was taken: the proxy answers it
             * only once it has forgotten the conversation. */
            self.asked = None;
            self.seq = FIRST_SEQ;
            return true;
        }
        if lost(raw) {
            self.broke(Broke::Lost);
            return false;
        }
        let Some(a) = answer(raw) else {
            self.broke(Broke::Garbled);
            return false;
        };
        self.asked = None;
        self.pending.extend_from_slice(a.bytes);
        if a.closed {
            /* The far end finished; bytes not yet sent have nowhere to go. */
            self.closed = true;
            self.unsent.clear();
        }
        self.seq = next_seq(self.seq);
        true
    }

    /// A call carrying the last frame was refused outright: the proxy is not
    /// there to ask. What it already answered can still be read.
    pub fn refused(&mut self) {
        self.broke(Broke::Gone);
    }

    /// Nothing more can be asked, for `why`. What was answered can still be
    /// read.
    fn broke(&mut self, why: Broke) {
        self.broken = true;
        self.why = Some(why);
        self.asked = None;
        self.unsent.clear();
    }

    /// Why the proxy can be asked nothing more, if that is not the far end
    /// finishing.
    pub fn why_broken(&self) -> Option<Broke> {
        self.why
    }

    /// Move up to `out.len()` answered bytes into `out`.
    pub fn read(&mut self, out: &mut [u8]) -> usize {
        let n = out.len().min(self.pending.len());
        out[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        n
    }
}
