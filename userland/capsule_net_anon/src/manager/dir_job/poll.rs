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

//! Advancing a fetch as far as it will go without waiting.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::mk_uptime_ms;

use crate::tcp_client::close;

use super::finish::finish;
use super::job::{Job, Stage};

/// What one stage did this turn.
pub(super) enum Progress {
    /// Finished; the next stage may run in the same turn.
    Next,
    /// Waiting on the network; come back next turn.
    Wait,
    /// Given up, already traced.
    Fail,
    /// The whole response has arrived.
    Done,
}

/// What a fetch has come to.
pub(super) enum Poll {
    Pending,
    /// The inflated body.
    Done(Vec<u8>),
    Failed,
}

impl Job {
    /// Run the fetch until it has to wait. The connection is closed once it
    /// is done or has failed, and never left open behind a result.
    pub(super) fn poll(&mut self, tcp_port: u32) -> Poll {
        let now = mk_uptime_ms();
        loop {
            let progress = match self.stage {
                Stage::Opening => self.opening(tcp_port, now),
                Stage::Sending => self.sending(tcp_port, now),
                Stage::Reading => self.reading(tcp_port, now),
            };
            match progress {
                Progress::Next => {}
                Progress::Wait => return Poll::Pending,
                Progress::Fail => break,
                Progress::Done => {
                    let _ = close(tcp_port, self.handle);
                    return finish(&self.raw, self.address, self.dir_port)
                        .map_or(Poll::Failed, Poll::Done);
                }
            }
        }
        let _ = close(tcp_port, self.handle);
        Poll::Failed
    }
}
