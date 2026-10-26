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

//! What either side of a pair can do now, in poll's bits, and what a read
//! would find, as FIONREAD counts it. Pure.

use super::input::MOST;
use super::out::OUT_MOST;
use super::pair::Pair;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLERR: u16 = 0x008;
const POLLHUP: u16 = 0x010;

impl Pair {
    /// A master is readable while it holds output, writable while the slave
    /// has room, and hung up once every slave has closed. A slave is
    /// readable once a read would not wait, and everything once hung up.
    pub fn bits(&self, master: bool) -> u16 {
        let on = |c: bool, bit: u16| if c { bit } else { 0 };
        if master {
            let gone = self.slave_seen && !self.slave;
            let room = self.input.ready.len() < MOST && self.output.len() < OUT_MOST;
            return on(!self.output.is_empty(), POLLIN) | on(room, POLLOUT) | on(gone, POLLHUP);
        }
        if self.hung {
            return POLLIN | POLLOUT | POLLERR | POLLHUP;
        }
        /* An end of file is a line of no bytes, which canonical mode reads. */
        let readable =
            !self.input.ready.is_empty() || (self.canon() && !self.input.ends.is_empty());
        on(readable, POLLIN) | on(self.output.len() < OUT_MOST, POLLOUT)
    }

    /// Bytes a read on this side would find now.
    pub fn queued(&self, master: bool) -> usize {
        match master {
            true => self.output.len(),
            false => self.input.ready.len(),
        }
    }
}
