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

//! Telling the program in front that the screen changed size.

use super::{attach_streams, streams, ALL};
use crate::jobs::JobWork;
use crate::term::state::State;

/// The screen changed size: the program in front hears the new one the
/// next time it asks.
pub fn resized(state: &State) {
    let Some(id) = state.jobs.foreground() else { return };
    let (pid, streams) = match state.jobs.get(id).map(|j| &j.work) {
        Some(JobWork::ExternalStage { pid, capture, stdin }) => {
            (*pid, streams(capture.is_some(), !stdin.takes_keys()))
        }
        Some(JobWork::InstallDrain(job)) if job.pid != 0 => (job.pid, ALL),
        _ => return,
    };
    attach_streams(state, pid, streams);
}
