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

//! A step's error: the request it asked for taken up to carry, or the clone's answer.

use nonos_git::TransportError;

use super::super::fail::say_failure;
use super::{CloneJob, Step, COMMAND};
use crate::command::output::Output;
use crate::jobs::JobProgress;
use crate::mixnet::Exchange;

/// What a remote can make the terminal allocate for one request: a depth-1
/// clone of this kernel is a 33 MB pack (`git::transport::round_trip`).
const MAX_RESPONSE: usize = 64 * 1024 * 1024;

impl CloneJob {
    /// The step asked for something not yet here: carry it, and run the
    /// step again once it has come. Any other error is the clone's answer.
    pub(super) fn ask_or_fail(
        &mut self,
        out: &mut Output<'_>,
        again: Step,
        e: TransportError,
    ) -> JobProgress {
        let Some(request) = self.transport.ask.take_asked() else {
            return self.fail(out, e);
        };
        let host = self.transport.remote.host.clone();
        let exchange =
            Exchange::new(self.route, &host, 443, true, self.rtc, request.clone(), MAX_RESPONSE);
        self.carrying = Some((request, exchange));
        self.reported = 0;
        self.step = Some(again);
        JobProgress::Running
    }

    pub(super) fn fail(&mut self, out: &mut Output<'_>, e: TransportError) -> JobProgress {
        say_failure(out, COMMAND, e, None);
        JobProgress::Done(1)
    }
}
