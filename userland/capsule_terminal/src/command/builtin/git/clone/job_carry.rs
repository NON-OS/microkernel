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

//! The request a clone step asked for, carried over the chosen network a tick at a time.

use nonos_git::TransportError;
use nonos_http::parse_response;

use super::super::fail::say_failure;
use super::{CloneJob, COMMAND};
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::mixnet::Poll;

impl CloneJob {
    pub(super) fn carry(&mut self, out: &mut Output<'_>) -> JobProgress {
        let Some((_, exchange)) = self.carrying.as_mut() else {
            return JobProgress::Running;
        };
        let raw = match exchange.step() {
            Poll::Pending => {
                self.report(out);
                return JobProgress::Running;
            }
            Poll::Ready(Ok(raw)) => raw,
            Poll::Ready(Err(why)) => {
                self.carrying = None;
                say_failure(out, COMMAND, TransportError::Unreachable, Some(why));
                return JobProgress::Done(1);
            }
        };
        let Some((request, _)) = self.carrying.take() else {
            return JobProgress::Done(1);
        };
        /*
         * A status other than 200 is an error rather than a body: git's own
         * error pages are valid HTTP and would otherwise be read as a pack.
         */
        let response = match parse_response(&raw) {
            Ok(response) => response,
            Err(_) => return self.fail(out, TransportError::Malformed),
        };
        if response.status != 200 {
            return self.fail(out, TransportError::Status(response.status));
        }
        self.transport.ask.answered(request, response.body);
        JobProgress::Running
    }
}
