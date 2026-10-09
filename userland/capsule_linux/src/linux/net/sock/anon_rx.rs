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

//! Reading a net.anon stream. net.anon has no way to ask whether bytes are
//! waiting without taking them, so poll and read both take what has arrived
//! into the socket's own buffer while it is empty, and a read drains it. The
//! buffer holds at most one reply's bytes (PAYLOAD_MAX): it is filled only
//! when empty, and a fill longer than that is refused as a broken answer.

use alloc::vec::Vec;

use crate::linux::abi::errno::{EAGAIN, ECONNREFUSED, ECONNRESET, EIO, ENETUNREACH, ETIMEDOUT};

use super::super::anon_ops::{PAYLOAD_MAX, REASON_DESTROY, REASON_DONE, REASON_TIMEOUT};
use super::backend::{Anon, End};

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLERR: u16 = 0x008;
const POLLHUP: u16 = 0x010;
const POLLRDHUP: u16 = 0x2000;

/// What one read from net.anon brought.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Got {
    Bytes(Vec<u8>),
    /// Nothing yet; the stream is open.
    Nothing,
    /// The far end finished, with this END reason.
    End(u8),
    /// net.anon holds no such stream for this capsule.
    Gone,
    /// An answer this capsule cannot read.
    Garbled,
    /// No answer.
    Silent,
}

impl Anon {
    /// Take in what a read from net.anon brought. Only an empty buffer on a
    /// stream still open takes anything, so bytes are never put behind
    /// bytes, or after the end.
    pub fn fill(&mut self, got: Got) {
        if !self.wants_fill() {
            return;
        }
        let end = match got {
            Got::Bytes(b) if b.len() <= PAYLOAD_MAX => {
                self.heard |= !b.is_empty();
                self.rx = b;
                return;
            }
            Got::Nothing => return,
            Got::End(REASON_DONE) => End::Eof,
            Got::End(reason) => End::Error(self.ended(reason)),
            Got::Gone => End::Error(ECONNRESET),
            Got::Bytes(_) | Got::Garbled | Got::Silent => End::Error(EIO),
        };
        self.end = Some(end);
    }

    /*
     * A stream net.anon ended before a byte came back was never connected:
     * its circuit went (no route), the exit gave up on the host, or the exit
     * or the host refused it. One that ended after was reset.
     */
    fn ended(&self, reason: u8) -> i64 {
        match reason {
            _ if self.heard => ECONNRESET,
            REASON_DESTROY => ENETUNREACH,
            REASON_TIMEOUT => ETIMEDOUT,
            _ => ECONNREFUSED,
        }
    }

    /// What a read of `want` bytes finds: held bytes, end of file once the
    /// read side is shut or the far end finished, the far end's error once,
    /// or EAGAIN while the stream is open and nothing has come.
    pub fn take(&mut self, want: usize, peek: bool, rd_shut: bool) -> Result<Vec<u8>, i64> {
        if want == 0 || rd_shut {
            return Ok(Vec::new());
        }
        if !self.rx.is_empty() {
            let n = want.min(self.rx.len());
            let out = self.rx[..n].to_vec();
            if !peek {
                self.rx.drain(..n);
            }
            return Ok(out);
        }
        match self.end {
            /* Reported once, as Linux reports a socket's error. */
            Some(End::Error(e)) => {
                self.end = Some(End::Eof);
                Err(e)
            }
            Some(End::Eof) => Ok(Vec::new()),
            None if self.id.is_none() => Ok(Vec::new()),
            None => Err(EAGAIN),
        }
    }

    /// poll's bits, as Linux's tcp_poll gives them. Readable only when
    /// bytes are held or the end has been seen, never on an empty buffer of
    /// a stream still open; always writable, since a write either goes,
    /// waits on net.anon's window (EAGAIN), or fails at once.
    pub fn bits(&self, rd_shut: bool, wr_shut: bool) -> u16 {
        let done = self.end.is_some() || self.id.is_none() || rd_shut;
        let broken = matches!(self.end, Some(End::Error(_)));
        let mut set = POLLOUT;
        if !self.rx.is_empty() || done {
            set |= POLLIN;
        }
        if done {
            set |= POLLRDHUP;
        }
        /* A reset closes both ways, as shutting both does. */
        if broken || (rd_shut && wr_shut) {
            set |= POLLHUP;
        }
        if broken {
            set |= POLLERR;
        }
        set
    }
}
