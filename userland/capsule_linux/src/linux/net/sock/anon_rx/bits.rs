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

//! What poll says of a net.anon stream.

use super::super::backend::{Anon, End};

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLERR: u16 = 0x008;
const POLLHUP: u16 = 0x010;
const POLLRDHUP: u16 = 0x2000;

impl Anon {
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
