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
 * Bytes through a `net.socks5` stream: each write carries bytes to the exit
 * and brings back whatever has come the other way; a read with nothing
 * waiting asks the proxy whether more has arrived.
 */

use alloc::vec::Vec;

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::socks::SocksLink;
use super::socks_call::{call, BYTES, CLOSED};

/* How long the proxy may take to answer the greeting and the CONNECT. */
const OPEN_MS: i64 = 60_000;

impl SocksLink {
    /* The next `n` bytes the proxy answers, waited for. */
    pub(super) fn answer(&mut self, n: usize) -> Result<Vec<u8>, ()> {
        let until = mk_uptime_ms().saturating_add(OPEN_MS);
        while self.pending.len() < n {
            if self.closed || mk_uptime_ms() > until {
                return Err(());
            }
            self.ask(&[BYTES])?;
            mk_idle_ms(20);
        }
        Ok(self.pending.drain(..n).collect())
    }

    fn ask(&mut self, framed: &[u8]) -> Result<(), ()> {
        let answer = call(self.port, framed)?;
        self.pending.extend_from_slice(&answer[1..]);
        self.closed |= answer[0] == CLOSED;
        Ok(())
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), ()> {
        for chunk in data.chunks(16 * 1024) {
            self.ask(&[&[BYTES][..], chunk].concat())?;
        }
        Ok(())
    }

    pub fn read(&mut self, into: &mut [u8]) -> Result<usize, ()> {
        if self.pending.is_empty() && !self.closed {
            self.ask(&[BYTES])?;
        }
        if self.pending.is_empty() {
            return if self.closed { Err(()) } else { Ok(0) };
        }
        let n = into.len().min(self.pending.len());
        into[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        Ok(n)
    }
}
