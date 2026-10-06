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

//! `shutdown` of a stream net.anon carries. net.anon ends a stream both
//! ways at once (RELAY_END) and has no half-close, so shutting the writing
//! side alone is refused, as it is for net.sockets' streams. Shutting the
//! reading side is the guest's own: later reads find end of file. Shutting
//! both closes the stream at net.anon, once; the socket and its descriptor
//! stay until closed, and freeing the socket then sends nothing more.

use crate::linux::abi::errno::EOPNOTSUPP;

use super::backend::{Anon, Backend};
use super::types::Sock;

impl Sock {
    pub fn shut_anon(&mut self, rd: bool, wr: bool) -> Result<(), i64> {
        if wr && !rd {
            return Err(EOPNOTSUPP);
        }
        self.rd_shut |= rd;
        if wr {
            self.wr_shut = true;
            if let Some(c) = self.svc.as_mut().and_then(Backend::anon_mut).and_then(Anon::close) {
                super::super::anon_stream::close(c);
            }
        }
        Ok(())
    }
}
