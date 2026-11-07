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


//! What a job's tick can change on screen, taken before and after it.

use crate::term::state::State;

use super::table::JobState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Seen {
    /// Scrollback writes so far (term/scrollback/types.rs).
    fed: u64,
    jobs: usize,
    running: usize,
}

impl Seen {
    pub fn of(state: &State) -> Self {
        let running = state.jobs.iter().filter(|j| j.state == JobState::Running).count();
        Seen { fed: state.scrollback.fed, jobs: state.jobs.iter().count(), running }
    }
}
