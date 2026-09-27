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

//! Settling reads parked on empty pipes.

use nonos_libc::mk_foreign_reply;

use super::family::Family;
use crate::linux::guest::Kind;

impl Family {
    /// Give each parked pipe read its bytes, or end of file once no process
    /// in the family holds a write end.
    pub fn settle_pipes(&mut self) {
        for i in 0..self.guests.len() {
            let Some((slot, buf, len, tid)) = self.guests[i].pipe_wait else {
                continue;
            };
            let have = self.pipes.get(slot).map_or(0, |p| p.len());
            let value = if have > 0 {
                let take = (len as usize).min(have);
                let bytes: alloc::vec::Vec<u8> = self.pipes[slot].drain(..take).collect();
                match self.guests[i].write(buf, &bytes) < take as i64 {
                    true => crate::linux::abi::errno::fail(crate::linux::abi::errno::EFAULT),
                    false => take as u64,
                }
            } else if !self.writer_left(slot) {
                0
            } else {
                continue;
            };
            self.guests[i].pipe_wait = None;
            let _ = mk_foreign_reply(tid, value);
        }
    }

    fn writer_left(&self, slot: usize) -> bool {
        self.guests.iter().any(|g| {
            g.fds.iter().any(|f| f.kind == Kind::Pipe && f.writable && f.handle as usize == slot)
        })
    }
}
