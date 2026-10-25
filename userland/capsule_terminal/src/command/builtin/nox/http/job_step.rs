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

//! An `http` job a tick at a time: each new stage said once, then the answer or why not.

use alloc::vec::Vec;

use nonos_http::parse_response;

use super::super::emit::emit;
use super::HttpJob;
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::mixnet::{Poll, Stage};

impl HttpJob {
    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        let raw = match self.exchange.step() {
            Poll::Pending => {
                self.tell(out);
                return JobProgress::Running;
            }
            Poll::Ready(Ok(raw)) => raw,
            Poll::Ready(Err(why)) => return fail(out, why),
        };
        match parse_response(&raw) {
            Ok(response) => {
                emit(out, &response);
                JobProgress::Done(0)
            }
            Err(_) => fail(out, "malformed response"),
        }
    }

    /// One line as the request reaches a new stage, so a slow network shows
    /// what it is waiting on rather than nothing.
    fn tell(&mut self, out: &mut Output<'_>) {
        let stage = self.exchange.stage();
        if self.shown == Some(stage) {
            return;
        }
        self.shown = Some(stage);
        match stage {
            Stage::Handshake => out.writeln(b"http: securing the connection"),
            Stage::Receiving => out.writeln(b"http: waiting for the answer"),
            Stage::Connecting | Stage::Sending => {}
        }
    }
}

fn fail(out: &mut Output<'_>, why: &str) -> JobProgress {
    let mut line = Vec::from(&b"http: "[..]);
    line.extend_from_slice(why.as_bytes());
    out.writeln_error(&line);
    JobProgress::Done(1)
}
