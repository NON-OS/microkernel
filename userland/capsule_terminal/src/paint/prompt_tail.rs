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

//! The prompt's segments after the path: ` main` when the shell is in a git
//! work tree, ` 2 jobs` while jobs run in the background.

use nonos_app_skeleton::PaintBuffer;

use super::line_text::text;
use super::metrics::Metrics;
use crate::jobs::JobState;
use crate::term::prompt_bits::{fit, jobs_label};
use crate::term::state::State;
use crate::term::theme::types::Theme;

/// The segments that fit after the path; either may be empty.
pub(super) struct Tail<'a> {
    pub branch: &'a [u8],
    pub jobs: &'a [u8],
}

/// What fits beside the path, from the state's branch and background jobs.
pub(super) fn tail<'a>(
    state: &'a State,
    buf: &'a mut [u8; 12],
    room: usize,
    path: usize,
) -> Tail<'a> {
    let n = state.jobs.iter().filter(|j| j.background && j.state == JobState::Running).count();
    let jobs = jobs_label(n, buf);
    let (b, j) = fit(room, path, state.git_branch.len(), jobs.len());
    Tail { branch: if b { &state.git_branch } else { &[] }, jobs: if j { jobs } else { &[] } }
}

/// Draw the tail from `x`, a cell after the path; say how many cells it took.
pub(super) fn draw_tail(
    fb: &mut PaintBuffer,
    tl: &Tail,
    x: u32,
    y: u32,
    m: Metrics,
    t: &Theme,
) -> usize {
    let mut cells = 0;
    for (seg, tone) in [(tl.branch, t.accent), (tl.jobs, t.dim)] {
        if seg.is_empty() {
            continue;
        }
        cells += 1;
        text(fb, x + cells as u32 * m.adv, y, seg, tone, m.adv, m.px);
        cells += seg.len();
    }
    cells
}
