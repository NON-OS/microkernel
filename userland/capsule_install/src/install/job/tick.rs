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

//! One tick: one budget of the current phase. The write hands over to the
//! read-back when the receipt arrives; the read-back hands over to the done
//! screen when the last file matches. Either hands over to the failed
//! screen on the first error, with the disk left exactly as far as it got.

use nonos_disk::{Progress, Verifier};
use nonos_libc::mk_time_millis;

use super::finish::finish;
use super::start::describe;
use super::work::{Job, Phase};
use crate::install::state::{Screen, State};

/// True when something on screen changed.
pub fn tick(state: &mut State) -> bool {
    let Some(job) = state.job.as_mut() else { return false };
    let now = mk_time_millis().max(0) as u64;
    let result = match &mut job.phase {
        Phase::Writing(session) => match session.step(&mut job.sink, Job::BUDGET) {
            Ok(Progress::Writing { done, .. }) => {
                job.done = done;
                Ok(None)
            }
            Ok(Progress::TableWritten) => Ok(None),
            Ok(Progress::Done(receipt)) => {
                job.write_seconds = now.saturating_sub(job.started_ms) / 1000;
                let verifier = Verifier::new(&receipt);
                job.total = verifier.total_bytes();
                job.done = 0;
                job.receipt = Some(receipt);
                job.phase = Phase::Verifying(verifier);
                state.screen = Screen::Verifying;
                Ok(None)
            }
            Err(e) => Err(e),
        },
        Phase::Verifying(verifier) => match verifier.step(&mut job.sink, Job::BUDGET) {
            Ok(true) => {
                job.done = verifier.checked;
                Ok(None)
            }
            Ok(false) => Ok(Some(())),
            Err(e) => Err(e),
        },
    };
    match result {
        Ok(None) => true,
        Ok(Some(())) => {
            finish(state, None);
            true
        }
        Err(e) => {
            finish(state, Some(describe(e)));
            true
        }
    }
}
