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

//! Handing the request to the socket, as much as it takes per turn.

use crate::tcp_client::send_some;
use crate::trace;

use super::job::{Job, Stage, READ_MS};
use super::poll::Progress;

impl Job {
    pub(super) fn sending(&mut self, tcp_port: u32, now: i64) -> Progress {
        while self.sent < self.request.len() {
            match send_some(tcp_port, self.handle, &self.request[self.sent..]) {
                Ok(0) if now >= self.deadline => break,
                Ok(0) => return Progress::Wait,
                Ok(took) => self.sent += took,
                Err(_) => break,
            }
        }
        if self.sent < self.request.len() {
            trace::say_addr(b"dir request not sent", self.address, self.dir_port);
            return Progress::Fail;
        }
        self.stage = Stage::Reading;
        self.deadline = now.saturating_add(READ_MS);
        Progress::Next
    }
}
