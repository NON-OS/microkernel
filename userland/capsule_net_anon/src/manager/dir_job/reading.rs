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

//! Reading the response until the far end closes, a bounded amount per turn.

extern crate alloc;

use alloc::vec;

use crate::tcp_client::recv;

use super::super::body_closed::finished;
use super::super::body_step::{step, Step};
use super::job::Job;
use super::poll::Progress;

const CHUNK: usize = 16 * 1024;

/// Reads per turn. What is waiting is taken without handing the turn back,
/// up to this, so a large document neither starves callers nor crawls.
const READS_PER_TURN: usize = 8;

/*
 * The request asks for Connection: close, so the body ends when the far end
 * stops, and net.tcp answers an empty read for both "nothing yet" and
 * "closed"; the socket state tells them apart.
 */
impl Job {
    pub(super) fn reading(&mut self, tcp_port: u32, now: i64) -> Progress {
        let mut chunk = vec![0u8; CHUNK];
        for _ in 0..READS_PER_TURN {
            let Ok(read) = recv(tcp_port, self.handle, &mut chunk) else {
                return Progress::Fail;
            };
            let closed = read == 0 && finished(tcp_port, self.handle);
            match step(read, closed, now >= self.deadline) {
                Step::Keep => self.raw.extend_from_slice(&chunk[..read]),
                /*
                 * A peer that has sent its FIN may still have bytes queued
                 * here, so the close is drained, and what it yields is kept.
                 */
                Step::Drain => match recv(tcp_port, self.handle, &mut chunk) {
                    Ok(0) => return self.ended(),
                    Ok(more) => self.raw.extend_from_slice(&chunk[..more]),
                    Err(_) => return self.ended(),
                },
                Step::Stop => return self.ended(),
                Step::Wait => return Progress::Wait,
            }
        }
        Progress::Wait
    }

    fn ended(&self) -> Progress {
        if self.raw.is_empty() {
            Progress::Fail
        } else {
            Progress::Done
        }
    }
}
