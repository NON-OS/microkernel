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

//! Bytes into a family stream: to its peer's queue.

use core::mem;

use crate::linux::abi::errno::{EAGAIN, EBADF, EPIPE};

use super::table::Socks;
use super::types::Domain;

impl Socks {
    /// As many of `bytes` as the peer has room for, EAGAIN for none.
    pub fn write(&mut self, id: u32, bytes: &[u8]) -> Result<usize, i64> {
        let s = self.get_mut(id).ok_or(EBADF)?;
        if s.error != 0 {
            return Err(mem::take(&mut s.error));
        }
        if !s.connected || s.wr_shut || s.broken {
            return Err(EPIPE);
        }
        let Some(p) = s.peer else {
            // TCP takes the first write after the peer left, and the reset
            // that answers it makes every later one EPIPE. A Unix socket
            // knows at once.
            if s.domain == Domain::Unix {
                return Err(EPIPE);
            }
            s.broken = true;
            return Ok(bytes.len());
        };
        let peer = self.get_mut(p).ok_or(EPIPE)?;
        let room = (peer.opts.rcvbuf as usize).saturating_sub(peer.rx.len());
        if room == 0 && !bytes.is_empty() {
            return Err(EAGAIN);
        }
        let n = room.min(bytes.len());
        peer.rx.extend(&bytes[..n]);
        Ok(n)
    }
}
