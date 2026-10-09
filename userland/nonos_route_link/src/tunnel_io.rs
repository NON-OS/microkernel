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
 * Bytes through an open tunnel. Each write carries bytes to the exit and
 * brings back whatever has come the other way. A read with nothing held
 * asks the proxy whether more has arrived, which is the only way a reply
 * that lands seconds after its request is ever collected: through either
 * network the answer is almost never ready inside the call that sent the
 * request.
 */

use crate::answer::{arrived, decode, Malformed};
use crate::bounds::{ASKS, ASK_GAP_MS, PENDING_MAX, POLL_GAP_MS, POLL_WAIT_MS, SEND_WAIT_MS};
use crate::carrier::Carrier;
use crate::frame::{next_seq, numbered, CARRY_MAX};
use crate::refusal::{FINISHED, OVERRUN, TOO_LONG};
use crate::tunnel::Tunnel;

impl<C: Carrier> Tunnel<C> {
    /*
     * One numbered exchange carrying `body`, which may be empty. When no
     * answer comes the same frame goes again, number and bytes alike, so
     * the proxy hands back the answer it kept rather than carrying the
     * bytes a second time; the number moves on only once one arrives.
     */
    pub(crate) fn exchange(&mut self, body: &[u8]) -> Result<(), &'static str> {
        let frame = numbered(self.seq, body).ok_or(TOO_LONG)?;
        let wait = if body.is_empty() { POLL_WAIT_MS } else { SEND_WAIT_MS };
        for _ in 0..ASKS {
            let got = self.carrier.call(&frame, wait, &mut self.rx);
            if got >= 0 {
                return self.absorb(got);
            }
            self.carrier.pause(ASK_GAP_MS);
        }
        Err(self.proxy.silent())
    }

    /*
     * Take one answer of `got` bytes into what the reader is owed. Its
     * length is checked against the buffer it was read into and against the
     * longest answer a proxy builds before a byte of it is used.
     */
    pub(crate) fn absorb(&mut self, got: i64) -> Result<(), &'static str> {
        let proxy = self.proxy;
        let n = arrived(got, self.rx.len()).ok_or(proxy.garbled())?;
        let answer = match decode(&self.rx[..n]) {
            Ok(answer) => answer,
            Err(Malformed::Lost) => {
                self.closed = true;
                return Err(proxy.lost());
            }
            Err(_) => return Err(proxy.garbled()),
        };
        if self.pending.len().saturating_add(answer.bytes.len()) > PENDING_MAX {
            return Err(OVERRUN);
        }
        self.pending.extend_from_slice(answer.bytes);
        self.closed |= answer.closed;
        self.seq = next_seq(self.seq);
        Ok(())
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), &'static str> {
        for chunk in data.chunks(CARRY_MAX) {
            if self.closed {
                return Err(FINISHED);
            }
            self.exchange(chunk)?;
        }
        Ok(())
    }

    /*
     * What has arrived, asking the proxy once when nothing is held. Zero
     * means nothing has come yet, or, once `ended`, that nothing will.
     */
    pub fn read(&mut self, into: &mut [u8]) -> Result<usize, &'static str> {
        if self.pending.is_empty() && !self.closed {
            self.exchange(&[])?;
        }
        let n = into.len().min(self.pending.len());
        into[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        Ok(n)
    }

    /*
     * What has arrived, waiting up to `wait_ms` for the first of it. Zero
     * after the wait, or at once when the far end has finished.
     */
    pub fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        if into.is_empty() {
            return Ok(0);
        }
        let wait = i64::try_from(wait_ms).unwrap_or(i64::MAX);
        let until = self.carrier.now_ms().saturating_add(wait);
        loop {
            let n = self.read(into)?;
            if n > 0 || self.ended() || self.carrier.now_ms() >= until {
                return Ok(n);
            }
            self.carrier.pause(POLL_GAP_MS);
        }
    }

    /* The far end finished and everything it sent has been read. */
    pub fn ended(&self) -> bool {
        self.closed && self.pending.is_empty()
    }
}
