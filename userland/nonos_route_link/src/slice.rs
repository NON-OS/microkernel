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

/*
 * A tunnel worked in slices, for a caller that must never wait long: a
 * window's own thread, which paints and takes input between slices. Each
 * call to the proxy waits at most the slice it is given. An exchange with
 * no answer in that time is not an error: the same number goes again with
 * the same bytes on a later slice, and the proxy hands back the answer it
 * kept, as it does for a caller whose answer was lost. Only an exchange
 * unanswered for as long as the blocking path would have tried it counts
 * as the proxy gone silent.
 */

use crate::bounds::{ASKS, POLL_WAIT_MS, REASK_GAP_MS, SEND_WAIT_MS};
use crate::carrier::Carrier;
use crate::frame::{numbered, CARRY_MAX};
use crate::refusal::{FINISHED, TOO_LONG};
use crate::tunnel::Tunnel;

/* What one slice of work came to. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Slice {
    /* Nothing yet; come back on a later slice. */
    Waiting,
    Done,
    Failed(&'static str),
}

/* The exchange asked and not yet answered: since when, and when last. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unanswered {
    since: i64,
    last: i64,
}

impl<C: Carrier> Tunnel<C> {
    /*
     * One slice of the exchange carrying `body`, which may be empty. Done
     * once the answer came and the number moved on. A caller that is told
     * Waiting must offer the same bytes again.
     */
    pub(crate) fn exchange_slice(&mut self, body: &[u8], slice_ms: u64) -> Slice {
        let now = self.carrier.now_ms();
        if let Some(u) = self.unanswered {
            if now.saturating_sub(u.last) < REASK_GAP_MS {
                return Slice::Waiting;
            }
        }
        let Some(frame) = numbered(self.seq, body) else {
            return Slice::Failed(TOO_LONG);
        };
        let got = self.carrier.call(&frame, slice_ms, &mut self.rx);
        if got >= 0 {
            self.unanswered = None;
            return match self.absorb(got) {
                Ok(()) => Slice::Done,
                Err(why) => Slice::Failed(why),
            };
        }
        let after = self.carrier.now_ms();
        let since = self.unanswered.map_or(now, |u| u.since);
        self.unanswered = Some(Unanswered { since, last: after });
        let wait = if body.is_empty() { POLL_WAIT_MS } else { SEND_WAIT_MS };
        let silent = i64::try_from(wait.saturating_mul(ASKS as u64)).unwrap_or(i64::MAX);
        if after.saturating_sub(since) >= silent {
            return Slice::Failed(self.proxy.silent());
        }
        Slice::Waiting
    }

    /*
     * What has arrived, asking the proxy at most once and for at most a
     * slice when nothing is held. Zero means nothing yet, or, once `ended`,
     * that nothing will come.
     */
    pub fn read_slice(&mut self, into: &mut [u8], slice_ms: u64) -> Result<usize, &'static str> {
        if self.pending.is_empty() && !self.closed {
            if let Slice::Failed(why) = self.exchange_slice(&[], slice_ms) {
                return Err(why);
            }
        }
        let n = into.len().min(self.pending.len());
        into[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        Ok(n)
    }

    /*
     * Carry the front of `data`, one frame's worth at most, for at most a
     * slice. How many bytes went: zero when the proxy has not answered yet,
     * and the caller offers the same bytes again.
     */
    pub fn write_slice(&mut self, data: &[u8], slice_ms: u64) -> Result<usize, &'static str> {
        if data.is_empty() {
            return Ok(0);
        }
        if self.closed {
            return Err(FINISHED);
        }
        let chunk = &data[..data.len().min(CARRY_MAX)];
        match self.exchange_slice(chunk, slice_ms) {
            Slice::Done => Ok(chunk.len()),
            Slice::Waiting => Ok(0),
            Slice::Failed(why) => Err(why),
        }
    }
}
