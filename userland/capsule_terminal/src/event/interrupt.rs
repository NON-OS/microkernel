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

//! Ctrl+C: stop the foreground job, or drop the line being typed.

use nonos_app_skeleton::EventOutcome;
use nonos_libc::mk_kill;

use crate::jobs::JobWork;
use crate::term::state::State;

const SIGINT: u64 = 2;

pub fn interrupt(state: &mut State) -> EventOutcome {
    if state.fg_running {
        if let Some(id) = state.jobs.foreground() {
            if let Some(job) = state.jobs.get_mut(id) {
                if let JobWork::ExternalStage { pid, .. } = job.work {
                    let _ = mk_kill(pid as u64, SIGINT);
                }
                job.cancel = true;
            }
        }
        state.cooked.line.clear();
    } else {
        state.line.clear();
        state.history.reset_cursor();
    }
    state.scrollback.push_line(b"^C");
    state.scrollback.jump_bottom();
    EventOutcome::Repaint
}
