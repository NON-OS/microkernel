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

//! Waiting for the connection to open, one look per turn.

use crate::tcp_client::{state, CLOSED, ESTABLISHED, SYN_RECEIVED, SYN_SENT};
use crate::trace;

use super::job::{Job, Stage, SEND_MS};
use super::poll::Progress;

impl Job {
    /*
     * net.tcp reports Closed until its interface is next polled, so the first
     * looks after connect say Closed for a socket that is about to open. Closed
     * only means refused once the socket has been seen on its way up.
     */
    pub(super) fn opening(&mut self, tcp_port: u32, now: i64) -> Progress {
        let refused = match state(tcp_port, self.handle) {
            Ok(ESTABLISHED) => {
                self.stage = Stage::Sending;
                self.deadline = now.saturating_add(SEND_MS);
                return Progress::Next;
            }
            Ok(SYN_SENT | SYN_RECEIVED) => {
                self.opening = true;
                false
            }
            Ok(CLOSED) => self.opening,
            _ => true,
        };
        if refused || now >= self.deadline {
            trace::say_addr(b"dir never established", self.address, self.dir_port);
            return Progress::Fail;
        }
        Progress::Wait
    }
}
