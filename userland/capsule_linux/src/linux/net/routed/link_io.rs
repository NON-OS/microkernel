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

//! A guest's reads and writes on an open stream through net.socks5, as
//! Linux's tcp_recvmsg and tcp_sendmsg answer them.

use alloc::vec::Vec;

use super::super::sock::End;
use super::link::{Link, TX_MAX};
use crate::linux::abi::errno::{EAGAIN, EPIPE};

impl Link {
    /// Queue the front of `bytes`: how many were taken, EAGAIN while the
    /// queue is full, EPIPE once a write has failed.
    pub fn send(&mut self, bytes: &[u8]) -> Result<usize, i64> {
        if self.broken {
            return Err(EPIPE);
        }
        let n = bytes.len().min(TX_MAX - self.tx.len());
        if n == 0 && !bytes.is_empty() {
            return Err(EAGAIN);
        }
        self.tx.extend_from_slice(&bytes[..n]);
        self.pump();
        Ok(n)
    }

    /// Up to `want` bytes: what has come, end of file once the far end
    /// finished, its error once, or EAGAIN while nothing has come.
    pub fn take(&mut self, want: usize, peek: bool) -> Result<Vec<u8>, i64> {
        if want == 0 {
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
            Some(End::Error(e)) => {
                self.end = Some(End::Eof);
                Err(e)
            }
            Some(End::Eof) => Ok(Vec::new()),
            None => Err(EAGAIN),
        }
    }
}
