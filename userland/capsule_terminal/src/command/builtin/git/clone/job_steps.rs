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

//! A clone job a tick at a time: which step runs, then the ref and pack steps.

use alloc::format;
use alloc::string::String;

use nonos_git::{discover, fetch, ObjectId, TransportError};

use super::{CloneJob, Step, DEPTH};
use crate::command::output::Output;
use crate::jobs::JobProgress;

impl CloneJob {
    pub fn step_once(&mut self, out: &mut Output<'_>) -> JobProgress {
        if self.carrying.is_some() {
            return self.carry(out);
        }
        match self.step.take() {
            Some(Step::Refs) => self.refs(out),
            Some(Step::Pack(head)) => self.pack(out, head),
            Some(Step::Write(head, pack)) => self.write(out, head, &pack),
            None => JobProgress::Done(1),
        }
    }

    fn refs(&mut self, out: &mut Output<'_>) -> JobProgress {
        match discover(&mut self.transport, "git-upload-pack") {
            Ok(refs) => {
                let mut full = String::from("refs/heads/");
                full.push_str(&self.branch);
                let Some(head) = refs.iter().find(|r| r.name == full).map(|r| r.id) else {
                    return self.fail(out, TransportError::Malformed);
                };
                let line = format!("git clone: fetching {} (depth {DEPTH})", self.branch);
                out.writeln(line.as_bytes());
                self.step = Some(Step::Pack(head));
                JobProgress::Running
            }
            Err(e) => self.ask_or_fail(out, Step::Refs, e),
        }
    }

    fn pack(&mut self, out: &mut Output<'_>, head: ObjectId) -> JobProgress {
        match fetch(&mut self.transport, &[head], DEPTH) {
            Ok(pack) => {
                let line =
                    format!("git clone: writing {} KiB into {}", pack.len() / 1024, self.into);
                out.writeln(line.as_bytes());
                self.step = Some(Step::Write(head, pack));
                JobProgress::Running
            }
            Err(e) => self.ask_or_fail(out, Step::Pack(head), e),
        }
    }
}
