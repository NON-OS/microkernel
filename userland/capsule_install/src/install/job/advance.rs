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

//! Moving a job on by one budget, and what that changed for the screen.

use alloc::string::String;

use nonos_disk::{Progress, Verifier, WriteError};

use super::start::describe;
use super::work::{Job, Phase};

pub enum Advanced {
    Step,
    /// The write is flushed and the read-back has begun.
    Verifying,
    /// The last sector read back as written.
    Done,
}

/// An error comes back in words, with what the bad sector holds when the
/// read-back names one.
pub fn advance(job: &mut Job, now: u64) -> Result<Advanced, String> {
    match &mut job.phase {
        Phase::Writing(session) => match session.step(&mut job.sink, Job::BUDGET) {
            Ok(Progress::Writing { done, .. }) => {
                job.done = done;
                Ok(Advanced::Step)
            }
            Ok(Progress::TableWritten) => Ok(Advanced::Step),
            Ok(Progress::Done(receipt)) => {
                job.write_seconds = now.saturating_sub(job.started_ms) / 1000;
                let verifier = Verifier::new(&receipt);
                job.total = verifier.total_bytes();
                job.done = 0;
                job.receipt = Some(receipt);
                job.phase = Phase::Verifying(verifier);
                Ok(Advanced::Verifying)
            }
            Err(e) => {
                job.status = sink_status(&e);
                Err(describe(e, Some(&session.plan().layout)))
            }
        },
        Phase::Verifying(verifier) => match verifier.step(&mut job.sink, Job::BUDGET) {
            Ok(true) => {
                job.done = verifier.checked;
                Ok(Advanced::Step)
            }
            Ok(false) => Ok(Advanced::Done),
            Err(e) => {
                job.status = sink_status(&e);
                Err(describe(e, job.receipt.as_ref().map(|r| &r.layout)))
            }
        },
    }
}

fn sink_status(e: &WriteError) -> Option<i32> {
    match e {
        WriteError::Sink(s) => Some(s.0),
        _ => None,
    }
}
