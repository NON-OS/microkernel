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

//! What a job's thread does: the work, then the state it leaves.

use shield_wire::Values;

use super::{Job, State, SLOT};

/// Run `work` as the job `name`, and leave what it answered in the slot.
pub(super) fn run<F>(name: &'static str, work: F)
where
    F: FnOnce() -> Result<Values, String>,
{
    crate::steps::serial(&format!("job {name}: running"));
    let state = match work() {
        Ok(v) => State::Done(String::from(v.text())),
        Err(why) => State::Failed(why),
    };
    crate::steps::serial(&format!(
        "job {name}: {}",
        if matches!(state, State::Done(_)) { "done" } else { "failed" }
    ));
    // A state request asked once this job is seen done reads it fresh.
    crate::ops::remember();
    crate::steps::serial(&format!("job {name}: state read"));
    if let Ok(mut slot) = SLOT.lock() {
        *slot = Some(Job { name, state });
    }
}
