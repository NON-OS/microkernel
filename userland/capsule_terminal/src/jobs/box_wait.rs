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

//! A boxed run's report waits for the shell's screen: a run in the
//! background that ends while another program holds the screen would draw
//! into that program's, so its report is kept until the shell has it back.

use super::box_hold::BoxHold;
use crate::command::output::Output;
use crate::jobs::JobProgress;

/// The status a run stopped by Ctrl+C ends with.
pub(super) const STOPPED: i32 = 130;

/// A run ended: a boxed one's report drawn now, or, while another program
/// holds the screen, kept until the shell has it back.
pub(super) fn ended(
    hold: Option<&mut BoxHold>,
    status: i32,
    out: &mut Output<'_>,
    held: bool,
) -> JobProgress {
    let Some(hold) = hold else { return JobProgress::Done(status) };
    hold.ended = Some(status);
    waiting(Some(hold), out, held).unwrap_or(JobProgress::Done(status))
}

/// A report kept for the shell's screen: drawn once it is, None if the run
/// has not ended.
pub(super) fn waiting(
    hold: Option<&mut BoxHold>,
    out: &mut Output<'_>,
    held: bool,
) -> Option<JobProgress> {
    let hold = hold?;
    let status = hold.ended?;
    if held {
        return Some(JobProgress::Running);
    }
    super::box_draw::draw(hold, out);
    Some(JobProgress::Done(status))
}

/// A run stopped by Ctrl+C: a report that came whole is drawn as at an end,
/// kept as one is; one that did not is said not to have come.
pub(super) fn stopped(hold: Option<&mut BoxHold>, out: &mut Output<'_>, held: bool) -> JobProgress {
    match hold {
        Some(hold) if !hold.complete() => {
            super::box_draw::missing(super::box_missing::Missing::Stopped, STOPPED, out);
            JobProgress::Done(STOPPED)
        }
        hold => ended(hold, STOPPED, out, held),
    }
}
